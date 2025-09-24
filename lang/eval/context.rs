// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    builtin::*, diag::*, eval::*, model::*, rc::*, resolve::*, syntax::*, tree_display::*,
};

/// Grant statements depending on context
pub trait Grant<T> {
    /// Check if given statement `T` is granted within the current context
    fn grant(&mut self, t: &T) -> EvalResult<()>;
}

/// *Context* for *evaluation* of a resolved µcad file.
///
/// The context is used to store the current state of the evaluation.
///
/// A context consists of the following members:
/// - A *symbol table* ([`SymbolTable`]) with symbols stored by [`QualifiedName`] and a [`Stack`].
/// - A *diagnostic handler* ([`DiagHandler`]) that accumulates *evaluation errors* for later output.
/// - One *output channel* ([`Output`]) where `__builtin::print` writes it's output to while evaluation.
///
/// All these internal structures can be accessed by several implemented traits.
pub struct Context {
    /// Symbol table
    symbol_table: SymbolTable,
    /// Source file diagnostics.
    diag_handler: DiagHandler,
    /// Output channel for [__builtin::print].
    output: Box<dyn Output>,
    /// Exporter registry.
    exporters: ExporterRegistry,
    /// Importer registry.
    importers: ImporterRegistry,
}

impl Context {
    /// Create a new context from a source file.
    ///
    /// # Arguments
    /// - `root`: Root symbol.
    /// - `symbols`: Pre-loaded symbols.
    /// - `sources`: Source file cache.
    /// - `output`: Output channel to use.
    pub fn new(
        root: Identifier,
        symbols: SymbolMap,
        sources: Sources,
        output: Box<dyn Output>,
    ) -> Self {
        log::debug!("Creating evaluation context");

        // put all together
        Self {
            symbol_table: SymbolTable::new(root, symbols, sources).expect("unknown root id"),
            output,
            diag_handler: Default::default(),
            exporters: ExporterRegistry::default(),
            importers: ImporterRegistry::default(),
        }
    }

    /// Current symbol, panics if there no current symbol.
    pub fn current_symbol(&self) -> Symbol {
        self.symbol_table
            .stack
            .current_symbol()
            .expect("Some symbol")
    }

    /// Create a new context from a source file.
    ///
    /// # Arguments
    /// - `root`: Path to the root file to load.
    /// - `builtin`: The builtin library.
    /// - `search_paths`: Paths to search for external libraries (e.g. the standard library).
    pub fn from_source(
        root: impl AsRef<std::path::Path> + std::fmt::Debug,
        builtin: Symbol,
        search_paths: &[std::path::PathBuf],
    ) -> EvalResult<Self> {
        let root = SourceFile::load(root)?;
        let root_id = root.id();
        let sources = Sources::default(); //load(root, search_paths)?;
        let mut symbols = sources.resolve()?;
        symbols.insert(Identifier::no_ref("__builtin"), builtin);
        Ok(Self::new(root_id, symbols, sources, Box::new(Stdout)))
    }

    /// Access captured output.
    pub fn output(&self) -> Option<String> {
        self.output.output()
    }

    /// Print for `__builtin::print`.
    pub fn print(&mut self, what: String) {
        self.output.print(what).expect("could not write to output");
    }

    /// Get the source code location of the given referrer as string (e.g. `/path/to/file.µcad:52:1`).
    pub fn locate(&self, referrer: &impl SrcReferrer) -> EvalResult<String> {
        Ok(format!(
            "{}:{}",
            self.get_by_hash(referrer.src_ref().source_hash())?
                .filename_as_str(),
            referrer.src_ref()
        ))
    }

    /// Get the original source code of the given referrer.
    pub fn source_code(&self, referrer: &impl SrcReferrer) -> EvalResult<String> {
        Ok(referrer
            .src_ref()
            .source_slice(&self.get_by_hash(referrer.src_ref().source_hash())?.source)
            .to_string())
    }

    /// Evaluate context into a value.
    pub fn eval(&mut self) -> EvalResult<Model> {
        let source_file = self.symbol_table.root.with_def(|def| match def {
            SymbolDefinition::SourceFile(source_file) => source_file.clone(),
            _ => todo!(),
        });
        source_file.eval(self)
    }

    /// Peek into root node for testing.
    pub fn root(&self) -> &Symbol {
        &self.symbol_table.root
    }

    /// Run the closure `f` within the given `stack_frame`.
    pub fn scope<T>(&mut self, stack_frame: StackFrame, f: impl FnOnce(&mut Context) -> T) -> T {
        self.open(stack_frame);
        let result = f(self);
        self.close();
        result
    }

    /// Set importers.
    pub fn set_importers(&mut self, importers: ImporterRegistry) {
        self.importers = importers;
    }

    /// All registered exporters.
    pub fn exporters(&self) -> &ExporterRegistry {
        &self.exporters
    }

    /// Set exporters.
    pub fn set_exporters(&mut self, exporters: ExporterRegistry) {
        self.exporters = exporters;
    }

    /// Return search paths of this context.
    pub fn search_paths(&self) -> &Vec<std::path::PathBuf> {
        self.symbol_table.search_paths()
    }

    /// Get property from current model.
    pub fn get_property(&self, id: &Identifier) -> EvalResult<Value> {
        match self.get_model() {
            Ok(model) => {
                if let Some(value) = model.get_property(id) {
                    Ok(value.clone())
                } else {
                    Err(EvalError::PropertyNotFound(id.clone()))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Initialize a property.
    ///
    /// Returns error if there is no model or the property has been initialized before.
    pub fn init_property(&self, id: Identifier, value: Value) -> EvalResult<()> {
        match self.get_model() {
            Ok(model) => {
                if let Some(previous_value) = model.borrow_mut().set_property(id.clone(), value) {
                    if !previous_value.is_invalid() {
                        return Err(EvalError::ValueAlreadyInitialized(
                            id.clone(),
                            previous_value,
                            id.src_ref(),
                        ));
                    }
                }
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    /// Return if the current frame is an init frame.
    pub fn is_init(&mut self) -> bool {
        matches!(
            self.symbol_table.stack.current_frame(),
            Some(StackFrame::Init(_))
        )
    }

    /// Lookup a property by qualified name.
    fn lookup_property(&self, name: &QualifiedName) -> EvalResult<Symbol> {
        match name.single_identifier() {
            Some(id) => match self.get_property(id) {
                Ok(value) => {
                    log::debug!(
                        "{found} property '{name:?}'",
                        found = crate::mark!(FOUND_INTERIM)
                    );
                    Ok(Symbol::new(
                        SymbolDefinition::Constant(Visibility::Public, id.clone(), value),
                        None,
                    ))
                }
                Err(err) => {
                    log::warn!(
                        "{not_found} Property '{name:?}'",
                        not_found = crate::mark!(NOT_FOUND_INTERIM)
                    );
                    Err(err)
                }
            },
            None => {
                log::debug!(
                    "{not_found} Property '{name:?}'",
                    not_found = crate::mark!(NOT_FOUND_INTERIM)
                );
                Err(EvalError::SymbolNotFound(name.clone()))
            }
        }
    }
}

impl Locals for Context {
    fn set_local_value(&mut self, id: Identifier, value: Value) -> EvalResult<()> {
        self.symbol_table.set_local_value(id, value)
    }

    fn get_local_value(&self, id: &Identifier) -> EvalResult<Value> {
        self.symbol_table.get_local_value(id)
    }

    fn open(&mut self, frame: StackFrame) {
        self.symbol_table.open(frame);
    }

    fn close(&mut self) {
        self.symbol_table.close();
    }

    fn fetch(&self, id: &Identifier) -> EvalResult<Symbol> {
        self.symbol_table.fetch(id)
    }

    fn get_model(&self) -> EvalResult<Model> {
        self.symbol_table.get_model()
    }

    fn current_name(&self) -> QualifiedName {
        self.symbol_table.current_name()
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            symbol_table: Default::default(),
            diag_handler: Default::default(),
            output: Box::new(Stdout),
            exporters: Default::default(),
            importers: Default::default(),
        }
    }
}

impl Lookup for Context {
    fn lookup(&mut self, name: &QualifiedName) -> EvalResult<Symbol> {
        log::debug!("Lookup symbol or property '{name}'");
        let symbol = self.symbol_table.lookup(name);
        let property = self.lookup_property(name);

        match (&symbol, &property) {
            (Ok(_), Err(_)) => {
                log::debug!(
                    "{found} symbol '{name:?}'",
                    found = crate::mark!(FOUND_FINAL)
                );
                symbol
            }
            (Err(_), Ok(_)) => {
                log::debug!(
                    "{found} property '{name:?}'",
                    found = crate::mark!(FOUND_FINAL)
                );
                property
            }
            (Ok(symbol), Ok(property)) => {
                log::debug!(
                    "{ambiguous} symbol '{name:?}' in {symbol} and {property}:\n{self}",
                    ambiguous = crate::mark!(AMBIGUOUS),
                );
                Err(EvalError::AmbiguousProperty(
                    symbol.full_name(),
                    property.id(),
                ))
            }
            // throw error from lookup on any error
            (Err(_), Err(_)) => {
                log::debug!(
                    "{not_found} symbol or property '{name:?}'",
                    not_found = crate::mark!(NOT_FOUND)
                );
                symbol
            }
        }
    }
}

impl Diag for Context {
    fn fmt_diagnosis(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result {
        self.diag_handler.pretty_print(f, &self.symbol_table)
    }

    fn error_count(&self) -> u32 {
        self.diag_handler.error_count()
    }

    fn error_lines(&self) -> std::collections::HashSet<usize> {
        self.diag_handler.error_lines()
    }

    fn warning_lines(&self) -> std::collections::HashSet<usize> {
        self.diag_handler.warning_lines()
    }
}

impl Context {
    /// use symbol in context
    pub fn use_symbol(
        &mut self,
        visibility: Visibility,
        name: &QualifiedName,
        id: Option<Identifier>,
    ) -> EvalResult<Symbol> {
        self.symbol_table
            .use_symbol(visibility, name, id, &self.current_name())
    }

    /// use all symbols of given module in context
    pub fn use_symbols_of(
        &mut self,
        visibility: Visibility,
        name: &QualifiedName,
    ) -> EvalResult<Symbol> {
        self.symbol_table
            .use_symbols_of(visibility, name, &self.current_name())
    }
}

impl PushDiag for Context {
    fn push_diag(&mut self, diag: Diagnostic) -> EvalResult<()> {
        let result = self.diag_handler.push_diag(diag);
        log::trace!("Error Context:\n{self}");
        result
    }
}

impl GetSourceByHash for Context {
    fn get_by_hash(&self, hash: u64) -> ResolveResult<Rc<SourceFile>> {
        self.symbol_table.get_by_hash(hash)
    }
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(model) = self.get_model() {
            write!(f, "\nModel:\n")?;
            model.tree_print(f, 4.into())?;
        }
        if self.has_errors() {
            writeln!(f, "{}\nErrors:", self.symbol_table)?;
            self.diag_handler.pretty_print(f, &self.symbol_table)?;
        } else {
            write!(f, "{}", self.symbol_table)?;
        }
        Ok(())
    }
}

impl ImporterRegistryAccess for Context {
    type Error = EvalError;

    fn import(
        &mut self,
        arg_map: &Tuple,
        search_paths: &[std::path::PathBuf],
    ) -> Result<Value, Self::Error> {
        match self.importers.import(arg_map, search_paths) {
            Ok(value) => Ok(value),
            Err(err) => {
                self.error(arg_map, err)?;
                Ok(Value::None)
            }
        }
    }
}

impl ExporterAccess for Context {
    fn exporter_by_id(&self, id: &crate::Id) -> Result<Rc<dyn Exporter>, ExportError> {
        self.exporters.exporter_by_id(id)
    }

    fn exporter_by_filename(
        &self,
        filename: &std::path::Path,
    ) -> Result<Rc<dyn Exporter>, ExportError> {
        self.exporters.exporter_by_filename(filename)
    }
}

impl Grant<WorkbenchDefinition> for Context {
    fn grant(&mut self, statement: &WorkbenchDefinition) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = self.symbol_table.stack.current_frame() {
            matches!(
                stack_frame,
                StackFrame::Source(_, _) | StackFrame::Module(_, _)
            )
        } else {
            false
        };
        if granted {
            Ok(())
        } else {
            self.error(
                statement,
                EvalError::StatementNotSupported(statement.kind.as_str()),
            )
        }
    }
}

impl Grant<ModuleDefinition> for Context {
    fn grant(&mut self, statement: &ModuleDefinition) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = self.symbol_table.stack.current_frame() {
            matches!(
                stack_frame,
                StackFrame::Source(_, _) | StackFrame::Module(_, _)
            )
        } else {
            false
        };
        if granted {
            Ok(())
        } else {
            self.error(statement, EvalError::StatementNotSupported("Module"))
        }
    }
}

impl Grant<FunctionDefinition> for Context {
    fn grant(&mut self, statement: &FunctionDefinition) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = self.symbol_table.stack.current_frame() {
            match stack_frame {
                // TODO: check if expression generates models (see test `source_expression``)
                StackFrame::Source(..) | StackFrame::Module(..) => true,
                StackFrame::Workbench(..) => statement.visibility == Visibility::Private,
                _ => false,
            }
        } else {
            false
        };
        if granted {
            Ok(())
        } else {
            self.error(statement, EvalError::StatementNotSupported("Function"))
        }
    }
}
impl Grant<InitDefinition> for Context {
    fn grant(&mut self, statement: &InitDefinition) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = self.symbol_table.stack.current_frame() {
            matches!(stack_frame, StackFrame::Workbench(..))
        } else {
            false
        };
        if granted {
            Ok(())
        } else {
            self.error(statement, EvalError::StatementNotSupported("Init"))
        }
    }
}

impl Grant<UseStatement> for Context {
    fn grant(&mut self, statement: &UseStatement) -> EvalResult<()> {
        match (
            &statement.visibility,
            self.symbol_table.stack.current_frame(),
        ) {
            (Visibility::Private, _) => Ok(()),
            (Visibility::Public, Some(StackFrame::Source(..) | StackFrame::Module(..))) => Ok(()),
            _ => self.error(statement, EvalError::StatementNotSupported("Use")),
        }
    }
}

impl Grant<ReturnStatement> for Context {
    fn grant(&mut self, statement: &ReturnStatement) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = self.symbol_table.stack.current_frame() {
            matches!(stack_frame, StackFrame::Function(_))
        } else {
            false
        };
        if granted {
            Ok(())
        } else {
            self.error(statement, EvalError::StatementNotSupported("Return"))
        }
    }
}

impl Grant<IfStatement> for Context {
    fn grant(&mut self, statement: &IfStatement) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = self.symbol_table.stack.current_frame() {
            matches!(
                stack_frame,
                StackFrame::Source(_, _)
                    | StackFrame::Workbench(_, _, _)
                    | StackFrame::Body(_)
                    | StackFrame::Function(_)
            )
        } else {
            false
        };
        if granted {
            Ok(())
        } else {
            self.error(statement, EvalError::StatementNotSupported("If"))
        }
    }
}

impl Grant<AssignmentStatement> for Context {
    fn grant(&mut self, statement: &AssignmentStatement) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = self.symbol_table.stack.current_frame() {
            match statement.assignment.qualifier {
                Qualifier::Const => {
                    matches!(stack_frame, StackFrame::Source(..) | StackFrame::Module(..))
                }
                Qualifier::Value => {
                    matches!(
                        stack_frame,
                        StackFrame::Source(..)
                            | StackFrame::Module(..)
                            | StackFrame::Body(_)
                            | StackFrame::Workbench(..)
                            | StackFrame::Init(_)
                            | StackFrame::Function(_)
                    )
                }
                Qualifier::Prop => matches!(stack_frame, StackFrame::Workbench(..)),
            }
        } else {
            false
        };
        if granted {
            Ok(())
        } else {
            self.error(statement, EvalError::StatementNotSupported("Assignment"))
        }
    }
}

impl Grant<ExpressionStatement> for Context {
    fn grant(&mut self, statement: &ExpressionStatement) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = self.symbol_table.stack.current_frame() {
            matches!(
                stack_frame,
                StackFrame::Source(_, _)
                    | StackFrame::Body(_)
                    | StackFrame::Workbench(_, _, _)
                    | StackFrame::Function(_)
            )
        } else {
            false
        };
        if granted {
            Ok(())
        } else {
            self.error(statement, EvalError::StatementNotSupported("Expression"))
        }
    }
}

impl Grant<Marker> for Context {
    fn grant(&mut self, statement: &Marker) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = self.symbol_table.stack.current_frame() {
            matches!(stack_frame, StackFrame::Workbench(_, _, _))
        } else {
            false
        };
        if granted {
            Ok(())
        } else {
            self.error(statement, EvalError::StatementNotSupported("Expression"))
        }
    }
}

impl Grant<crate::syntax::Attribute> for Context {
    fn grant(&mut self, statement: &crate::syntax::Attribute) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = self.symbol_table.stack.current_frame() {
            matches!(
                stack_frame,
                StackFrame::Source(_, _) | StackFrame::Body(_) | StackFrame::Workbench(_, _, _)
            )
        } else {
            false
        };
        if granted {
            Ok(())
        } else {
            self.error(
                statement,
                EvalError::StatementNotSupported("InnerAttribute"),
            )
        }
    }
}
