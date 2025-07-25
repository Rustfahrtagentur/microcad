// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{builtin::*, diag::*, eval::*, model::Model, rc::*, resolve::*, syntax::*};

/// Grant statements depending on context
pub trait Grant<T> {
    /// Check if given statement [`T`] is granted within the current context
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
    /// - `builtin`: The builtin library.
    /// - `search_paths`: Paths to search for external libraries (e.g. the standard library).
    /// - `output`: Output channel to use.
    pub fn new(
        root: Symbol,
        builtin: Symbol,
        search_paths: &[std::path::PathBuf],
        output: Box<dyn Output>,
    ) -> Self {
        log::debug!(
            "Creating Context (search paths: {})",
            search_paths
                .iter()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>()
                .join(",")
        );

        // put all together
        Self {
            symbol_table: SymbolTable::new(root, builtin, search_paths),
            diag_handler: Default::default(),
            output,
            exporters: ExporterRegistry::default(),
            importers: ImporterRegistry::default(),
        }
    }

    /// Create a new context from a source file.
    ///
    /// # Arguments
    /// - `root`: Path to the root file to load.
    /// - `builtin`: The builtin library.
    /// - `search_paths`: Paths to search for external libraries (e.g. the standard library).
    pub fn from_source(
        root: impl AsRef<std::path::Path>,
        builtin: Symbol,
        search_paths: &[std::path::PathBuf],
    ) -> EvalResult<Self> {
        Ok(Self::new(
            SourceFile::load(root)?.resolve(None),
            builtin,
            search_paths,
            Box::new(Stdout),
        ))
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
        let source_file = match &self.symbol_table.root.borrow().def {
            SymbolDefinition::SourceFile(source_file) => source_file.clone(),
            _ => todo!(),
        };
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
        self.symbol_table.lookup(name)
    }
}

impl Diag for Context {
    fn fmt_diagnosis(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result {
        self.diag_handler.pretty_print(f, &self.symbol_table)
    }

    fn error_count(&self) -> u32 {
        self.diag_handler.error_count()
    }
}

impl UseSymbol for Context {
    fn use_symbol(&mut self, name: &QualifiedName, id: Option<Identifier>) -> EvalResult<Symbol> {
        self.symbol_table.use_symbol(name, id)
    }

    fn use_symbols_of(&mut self, name: &QualifiedName) -> EvalResult<Symbol> {
        self.symbol_table.use_symbols_of(name)
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
    fn get_by_hash(&self, hash: u64) -> EvalResult<Rc<SourceFile>> {
        self.symbol_table.get_by_hash(hash)
    }
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.has_errors() {
            writeln!(f, "{}Errors:", self.symbol_table)?;
            self.diag_handler.pretty_print(f, &self.symbol_table)
        } else {
            write!(f, "{}", self.symbol_table)
        }
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
            matches!(
                stack_frame,
                // TODO: check if expression generates models (see test `source_expression``)
                StackFrame::Source(_, _)
                    | StackFrame::Module(_, _)
                    | StackFrame::Workbench(_, _, _)
            )
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
            matches!(stack_frame, StackFrame::Workbench(_, _, _))
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
        if self.symbol_table.stack.current_frame().is_some() {
            Ok(())
        } else {
            self.error(statement, EvalError::StatementNotSupported("Use"))
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
                Qualifier::Var => {
                    matches!(
                        stack_frame,
                        StackFrame::Source(_, _)
                            | StackFrame::Body(_)
                            | StackFrame::Workbench(_, _, _)
                            | StackFrame::Init(_)
                    )
                }
                Qualifier::Const => matches!(
                    stack_frame,
                    StackFrame::Source(_, _)
                        | StackFrame::Module(_, _)
                        | StackFrame::Workbench(_, _, _)
                ),
                Qualifier::Prop => matches!(stack_frame, StackFrame::Workbench(_, _, _)),
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
