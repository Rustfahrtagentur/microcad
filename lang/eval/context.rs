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
    /// Stack of currently opened scopes with symbols while evaluation.
    stack: Stack,
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
    /// - `builtin`: The builtin library.
    /// - `search_paths`: Paths to search for external libraries (e.g. the standard library).
    /// - `output`: Output channel to use.
    pub fn new(symbols: SymbolMap, sources: Sources, output: Box<dyn Output>) -> Self {
        log::debug!("Creating Context");

        // put all together
        Self {
            symbol_table: SymbolTable::new(symbols, sources, DiagHandler::default())
                .expect("unknown root id"),
            output,
            ..Default::default()
        }
    }

    /// Current symbol, panics if there no current symbol.
    pub fn current_symbol(&self) -> Symbol {
        self.stack.current_symbol().expect("Some symbol")
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
        search_paths: &[impl AsRef<std::path::Path>],
    ) -> EvalResult<Self> {
        let root = SourceFile::load(root)?;
        let sources = Sources::load(root, search_paths)?;
        let mut symbols = sources.resolve()?;
        symbols.insert(Identifier::no_ref("__builtin"), builtin);
        Ok(Self::new(symbols, sources, Box::new(Stdout)))
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
        let source_file = match &self.symbol_table.root().borrow().def {
            SymbolDefinition::SourceFile(source_file) => source_file.clone(),
            _ => todo!(),
        };
        source_file.eval(self)
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
        matches!(self.stack.current_frame(), Some(StackFrame::Init(_)))
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
                Err(EvalError::NoPropertyId(name.clone()))
            }
        }
    }

    /// Fetch local variable from local stack (for testing only).
    #[cfg(test)]
    pub fn fetch_local(&self, id: &Identifier) -> EvalResult<Symbol> {
        self.stack.fetch(id)
    }

    fn lookup_workbench(&self, name: &QualifiedName) -> ResolveResult<Symbol> {
        if let Some(workbench) = &self.stack.current_workbench_name() {
            log::trace!("Looking for symbol '{name:?}' in current workbench '{workbench:?}'");
            let name = &name.with_prefix(workbench);
            match self.symbol_table.lookup(name) {
                Ok(symbol) => {
                    if symbol.full_name() == *name {
                        log::trace!(
                            "{found} symbol in current module: {symbol}",
                            found = crate::mark!(FOUND),
                        );
                        return self.symbol_table.follow_alias(&symbol);
                    }
                }
                Err(err) => return Err(err)?,
            };
        }
        Err(ResolveError::SymbolNotFound(name.clone()))
    }

    fn lookup_within(&self, what: &QualifiedName, within: QualifiedName) -> EvalResult<Symbol> {
        log::trace!("Looking for symbol '{what:?}' within '{within:?}':",);

        // process internal supers
        let (what, within) = what.dissolve_super(within);

        let parents = self.symbol_table.path_to(&within)?;
        for (n, parent) in parents.iter().rev().enumerate() {
            log::trace!("  Looking in: {:?} for {:?}", parent.full_name(), what);
            if let Some(symbol) = parent.search(&what) {
                let alias = self.symbol_table.follow_alias(&symbol)?;
                if n > 0 {
                    if symbol.is_private() {
                        return Err(EvalError::SymbolIsPrivate {
                            what: what.clone(),
                            within,
                        });
                    }
                    if alias != symbol && alias.is_private() {
                        return Err(EvalError::SymbolBehindAliasIsPrivate {
                            what: what.clone(),
                            alias: alias.full_name(),
                            within,
                        });
                    }
                }
                return Ok(alias);
            }
        }
        Err(EvalError::SymbolNotFound(what.clone()))
    }
    /// Check if current stack frame is code
    pub fn is_code(&self) -> bool {
        !matches!(self.stack.current_frame(), Some(StackFrame::Module(..)))
    }

    /// Check if current stack frame is a module
    pub fn is_module(&self) -> bool {
        matches!(
            self.stack.current_frame(),
            Some(StackFrame::Module(..) | StackFrame::Source(..))
        )
    }
}
/*
impl UseSymbol for Context {
    fn use_symbol(
        &mut self,
        visibility: Visibility,
        name: &QualifiedName,
        id: Option<Identifier>,
        within: &QualifiedName,
    ) -> EvalResult<Symbol> {
        log::debug!("Using symbol {name:?}");

        let symbol = self.lookup(name)?;
        if self.is_module() {
            let id = id.clone().unwrap_or(symbol.id());
            let symbol = symbol.clone_with_visibility(visibility);
            if within.is_empty() {
                self.symbols.insert(id, symbol);
            } else {
                self.symbols
                    .search(within)?
                    .borrow_mut()
                    .children
                    .insert(id, symbol);
            }
            log::trace!("Symbol Table:\n{}", self.symbols);
        }

        if self.is_code() {
            self.stack.put_local(id, symbol.clone())?;
            log::trace!("Local Stack:\n{}", self.stack);
        }

        Ok(symbol)
    }

    fn use_symbols_of(
        &mut self,
        visibility: Visibility,
        name: &QualifiedName,
        within: &QualifiedName,
    ) -> EvalResult<Symbol> {
        log::debug!("Using all symbols in {name:?}");

        let symbol = self.lookup(name)?;
        if symbol.is_empty() {
            Err(EvalError::NoSymbolsToUse(symbol.full_name()))
        } else {
            if self.is_module() {
                for (id, symbol) in symbol.borrow().children.iter() {
                    let symbol = symbol.clone_with_visibility(visibility);
                    if within.is_empty() {
                        self.symbols.insert(id.clone(), symbol);
                    } else {
                        self.symbols
                            .search(within)?
                            .borrow_mut()
                            .children
                            .insert(id.clone(), symbol);
                    }
                }
                log::trace!("Symbol Table:\n{}", self.symbols);
            }

            if self.is_code() {
                for (id, symbol) in symbol.borrow().children.iter() {
                    self.stack.put_local(Some(id.clone()), symbol.clone())?;
                }
                log::trace!("Local Stack:\n{}", self.stack);
            }
            Ok(symbol)
        }
    }
}
*/
impl Locals for Context {
    fn set_local_value(&mut self, id: Identifier, value: Value) -> EvalResult<()> {
        self.stack.set_local_value(id, value)
    }

    fn get_local_value(&self, id: &Identifier) -> EvalResult<Value> {
        self.stack.get_local_value(id)
    }

    fn open(&mut self, frame: StackFrame) {
        self.stack.open(frame);
    }

    fn close(&mut self) {
        self.stack.close();
    }

    fn fetch(&self, id: &Identifier) -> EvalResult<Symbol> {
        self.stack.fetch(id)
    }

    fn get_model(&self) -> EvalResult<Model> {
        self.stack.get_model()
    }

    fn current_name(&self) -> QualifiedName {
        self.stack.current_name()
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            symbol_table: Default::default(),
            stack: Default::default(),
            output: Box::new(Stdout),
            exporters: Default::default(),
            importers: Default::default(),
        }
    }
}

impl Lookup<EvalError> for Context {
    fn lookup(&self, name: &QualifiedName) -> EvalResult<Symbol> {
        log::debug!("Lookup symbol '{name:?}' (at line {:?}):", name.src_ref());

        let name = &self.symbol_table.de_alias(name);

        log::trace!("- lookups -------------------------------------------------------");
        // collect all symbols that can be found and remember origin
        let result = [
            (
                "local",
                match self.stack.lookup(name) {
                    Ok(SymbolOrName::Name(name)) => self.lookup(&name),
                    Ok(SymbolOrName::Symbol(symbol)) => Ok(symbol),
                    Err(err) => Err(err),
                },
            ),
            (
                "module",
                self.lookup_within(name, self.stack.current_module_name()),
            ),
            ("property", self.lookup_property(name)),
            ("workbench", Ok(self.lookup_workbench(name)?)),
            ("global", Ok(self.symbol_table.lookup(name)?)),
        ]
        .into_iter();

        log::trace!("- result --------------------------------------------------------");
        let mut errors = Vec::new();

        // collect ok-results and ambiguity errors
        let (found, mut ambiguous) = result.fold(
            (Vec::new(), Vec::new()),
            |(mut oks, mut ambiguity), (origin, r)| {
                match r {
                    Ok(symbol) => oks.push((origin, symbol)),
                    Err(EvalError::AmbiguousSymbol { ambiguous, others }) => {
                        ambiguity.push((origin, EvalError::AmbiguousSymbol { ambiguous, others }))
                    }
                    Err(
                        EvalError::SymbolNotFound(_)
                        | EvalError::ResolveError(ResolveError::SymbolNotFound(_))
                        | EvalError::LocalNotFound(_)
                        | EvalError::ResolveError(ResolveError::ExternalPathNotFound(_))
                        | EvalError::ResolveError(ResolveError::NulHash),
                    ) => (),
                    Err(err) => errors.push((origin, err)),
                }
                (oks, ambiguity)
            },
        );

        // log any unexpected errors and return early
        if !errors.is_empty() {
            log::debug!("Unexpected errors while lookup symbol '{name:?}':");
            errors
                .iter()
                .for_each(|(origin, err)| log::error!("Lookup ({origin}) error: {err}"));

            return Err(errors.remove(0).1);
        }

        // early emit any ambiguity error
        if !ambiguous.is_empty() {
            log::debug!(
                "{ambiguous} Symbol '{name:?}':\n{}",
                ambiguous
                    .iter()
                    .map(|(origin, err)| format!("{origin}: {err}"))
                    .collect::<Vec<_>>()
                    .join("\n"),
                ambiguous = crate::mark!(AMBIGUOUS)
            );
            return Err(ambiguous.remove(0).1);
        }

        // follow aliases
        let found: Vec<_> = found
            .into_iter()
            .filter_map(|(origin, symbol)| {
                if let Ok(symbol) = self.symbol_table.follow_alias(&symbol) {
                    Some((origin, symbol))
                } else {
                    None
                }
            })
            .collect();

        // check for ambiguity in what's left
        match found.first() {
            Some((origin, first)) => {
                // check if all findings point to the same symbol
                if !found.iter().all(|(_, x)| Rc::ptr_eq(x, first)) {
                    log::debug!(
                        "{ambiguous} symbol '{name:?}' in {origin}:\n{self}",
                        ambiguous = crate::mark!(AMBIGUOUS),
                        origin = found
                            .iter()
                            .map(|(id, _)| *id)
                            .collect::<Vec<_>>()
                            .join(" and ")
                    );
                    Err(EvalError::AmbiguousSymbol {
                        ambiguous: name.clone(),
                        others: found.iter().map(|(_, x)| x.clone()).collect(),
                    })
                } else {
                    log::debug!(
                        "{found} symbol '{name:?}' in {origin}",
                        found = crate::mark!(FOUND_INTERIM)
                    );
                    Ok(first.clone())
                }
            }
            None => {
                log::debug!(
                    "{not_found} Symbol '{name:?}'",
                    not_found = crate::mark!(NOT_FOUND_INTERIM)
                );

                Err(EvalError::SymbolNotFound(name.clone()))
            }
        }
    }
}

/*
impl Lookup for Context {
    fn lookup(&self, name: &QualifiedName) -> EvalResult<Symbol> {
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
*/
impl Diag for Context {
    fn fmt_diagnosis(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result {
        self.symbol_table.fmt_diagnosis(f)
    }

    fn error_count(&self) -> u32 {
        self.symbol_table.error_count()
    }

    fn error_lines(&self) -> std::collections::HashSet<usize> {
        self.symbol_table.error_lines()
    }

    fn warning_lines(&self) -> std::collections::HashSet<usize> {
        self.symbol_table.warning_lines()
    }
}
/*
impl Context {
    /// use symbol in context
    pub fn use_symbol(
        &mut self,
        visibility: Visibility,
        name: &QualifiedName,
        id: Option<Identifier>,
    ) -> EvalResult<Symbol> {
        self.use_symbol(visibility, name, id, &self.current_name())
    }

    /// use all symbols of given module in context
    pub fn use_symbols_of(
        &mut self,
        visibility: Visibility,
        name: &QualifiedName,
    ) -> EvalResult<Symbol> {
        self.use_symbols_of(visibility, name, &self.current_name())
    }
}
*/
impl PushDiag for Context {
    fn push_diag(&mut self, diag: Diagnostic) -> EvalResult<()> {
        let result = self.symbol_table.push_diag(diag);
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
        writeln!(f, "\nCurrent: {}", self.stack.current_name())?;
        writeln!(f, "\nModule: {}", self.stack.current_module_name())?;
        write!(f, "\nLocals Stack:\n{}", self.stack)?;
        writeln!(f, "\nCall Stack:")?;
        self.stack
            .pretty_print_call_trace(f, &self.symbol_table.sources)?;

        if self.has_errors() {
            writeln!(f, "{}\nErrors:", self.symbol_table)?;
            self.symbol_table.fmt_diagnosis(f)?;
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
        let granted = if let Some(stack_frame) = self.stack.current_frame() {
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
        let granted = if let Some(stack_frame) = self.stack.current_frame() {
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
        let granted = if let Some(stack_frame) = self.stack.current_frame() {
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
        let granted = if let Some(stack_frame) = self.stack.current_frame() {
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
        match (&statement.visibility, self.stack.current_frame()) {
            (Visibility::Private, _) => Ok(()),
            (Visibility::Public, Some(StackFrame::Source(..) | StackFrame::Module(..))) => Ok(()),
            _ => self.error(statement, EvalError::StatementNotSupported("Use")),
        }
    }
}

impl Grant<ReturnStatement> for Context {
    fn grant(&mut self, statement: &ReturnStatement) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = self.stack.current_frame() {
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
        let granted = if let Some(stack_frame) = self.stack.current_frame() {
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
        let granted = if let Some(stack_frame) = self.stack.current_frame() {
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
        let granted = if let Some(stack_frame) = self.stack.current_frame() {
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
        let granted = if let Some(stack_frame) = self.stack.current_frame() {
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
        let granted = if let Some(stack_frame) = self.stack.current_frame() {
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
