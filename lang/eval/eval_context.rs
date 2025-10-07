// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    builtin::*, diag::*, eval::*, model::*, rc::*, resolve::*, syntax::*, tree_display::*,
};

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
pub struct EvalContext {
    /// Symbol table
    symbol_table: SymbolTable,
    /// Source cache
    sources: Sources,
    /// Stack of currently opened scopes with symbols while evaluation.
    pub(super) stack: Stack,
    /// Output channel for [__builtin::print].
    output: Box<dyn Output>,
    /// Exporter registry.
    exporters: ExporterRegistry,
    /// Importer registry.
    importers: ImporterRegistry,
    /// Diagnostics handler.
    diag: DiagHandler,
}

impl EvalContext {
    /// Create a new context from a resolved symbol table.
    pub fn new(
        resolve_context: ResolveContext,
        output: Box<dyn Output>,
        exporters: ExporterRegistry,
        importers: ImporterRegistry,
    ) -> Self {
        log::debug!("Creating Context");

        assert!(resolve_context.is_resolved());

        // put all together
        Self {
            symbol_table: resolve_context.symbol_table,
            sources: resolve_context.sources,
            diag: resolve_context.diag,
            output,
            exporters,
            importers,
            ..Default::default()
        }
    }

    /// Current symbol, panics if there no current symbol.
    pub fn current_symbol(&self) -> Symbol {
        self.stack.current_symbol().expect("Some symbol")
    }

    /// Create a new context from a source file.
    pub fn from_source(
        root: Rc<SourceFile>,
        builtin: Option<Symbol>,
        search_paths: &[impl AsRef<std::path::Path>],
        output: Box<dyn Output>,
        exporters: ExporterRegistry,
        importers: ImporterRegistry,
    ) -> EvalResult<Self> {
        Ok(Self::new(
            ResolveContext::create(
                root,
                search_paths,
                builtin,
                DiagHandler::default(),
                ResolveMode::Resolved,
            )?,
            output,
            exporters,
            importers,
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
        self.sources.root().eval(self)
    }

    /// Run the closure `f` within the given `stack_frame`.
    pub fn scope<T>(
        &mut self,
        stack_frame: StackFrame,
        f: impl FnOnce(&mut EvalContext) -> T,
    ) -> T {
        self.open(stack_frame);
        let result = f(self);
        self.close();
        result
    }

    /// All registered exporters.
    pub fn exporters(&self) -> &ExporterRegistry {
        &self.exporters
    }

    /// Return search paths of this context.
    pub fn search_paths(&self) -> &Vec<std::path::PathBuf> {
        self.sources.search_paths()
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
                        return Err(EvalError::ValueAlreadyDefined(
                            id.clone(),
                            previous_value.to_string(),
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
        log::trace!("looking for property {name:?}");

        match name.single_identifier() {
            Some(id) => match self.get_property(id) {
                Ok(value) => {
                    log::trace!(
                        "{found} property '{name:?}'",
                        found = crate::mark!(FOUND_INTERIM)
                    );
                    Ok(Symbol::new(
                        SymbolDefinition::Constant(Visibility::Public, id.clone(), value),
                        None,
                    ))
                }
                Err(err) => {
                    log::trace!(
                        "{not_found} Property '{name:?}'",
                        not_found = crate::mark!(NOT_FOUND_INTERIM)
                    );
                    Err(err)
                }
            },
            None => {
                log::trace!(
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
                            "{found} symbol in current module: {symbol:?}",
                            found = crate::mark!(FOUND),
                        );
                        return Ok(symbol);
                    }
                }
                Err(err) => return Err(err)?,
            };
        } else {
            log::trace!(
                "{not_found} No current workbench",
                not_found = crate::mark!(NOT_FOUND_INTERIM)
            );
        }
        Err(ResolveError::SymbolNotFound(name.clone()))
    }

    fn lookup_within(&self, what: &QualifiedName, within: QualifiedName) -> EvalResult<Symbol> {
        log::trace!("Looking for symbol '{what:?}' within '{within:?}':",);

        // process internal supers
        let (what, within) = what.dissolve_super(within);

        let parents = self.symbol_table.path_to(&within)?;
        for (n, parent) in parents.iter().rev().enumerate() {
            log::trace!("Looking in: {:?} for {:?}", parent.full_name(), what);
            if let Some(symbol) = parent.search(&what) {
                let alias = self.symbol_table.follow_alias(&symbol)?;
                if n > 0 {
                    if symbol.is_private() {
                        log::trace!(
                            "{not_found} symbol {what:?} within {within:?} is private",
                            not_found = crate::mark!(NOT_FOUND_INTERIM),
                        );
                        return Err(EvalError::SymbolIsPrivate {
                            what: what.clone(),
                            within,
                        });
                    }
                    if alias != symbol && alias.is_private() {
                        log::trace!(
                            "{not_found} within {within:?} symbol: {what:?}",
                            not_found = crate::mark!(NOT_FOUND_INTERIM),
                        );
                        return Err(EvalError::SymbolBehindAliasIsPrivate {
                            what: what.clone(),
                            alias: alias.full_name(),
                            within,
                        });
                    }
                }
                log::trace!(
                    "{found} symbol within {within:?}:` {alias:?}",
                    found = crate::mark!(FOUND),
                );
                return Ok(alias);
            }
        }
        log::trace!(
            "{not_found} within {within:?} symbol: {what:?}",
            not_found = crate::mark!(NOT_FOUND_INTERIM),
        );
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

impl UseSymbol for EvalContext {
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
                self.symbol_table.insert_symbol(id, symbol)?;
            } else {
                self.symbol_table.lookup(within)?.insert(id, symbol);
            }
            log::trace!("Symbol Table:\n{}", self.symbol_table);
        }

        if self.is_code() {
            self.stack.put_local(id, symbol.clone())?;
            log::trace!("Local Stack:\n{:?}", self.stack);
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
                symbol.with_children(|(id, symbol)| {
                    let symbol = symbol.clone_with_visibility(visibility);
                    if within.is_empty() {
                        self.symbol_table.insert_symbol(id.clone(), symbol)?;
                    } else {
                        self.symbol_table.lookup(within)?.insert(id.clone(), symbol);
                    }
                    Ok::<_, EvalError>(())
                })?;
                log::trace!("Symbol Table:\n{}", self.symbol_table);
            }

            if self.is_code() {
                symbol.with_children(|(id, symbol)| {
                    self.stack.put_local(Some(id.clone()), symbol.clone())
                })?;
                log::trace!("Local Stack:\n{:?}", self.stack);
            }
            Ok(symbol)
        }
    }
}

impl Locals for EvalContext {
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

impl Default for EvalContext {
    fn default() -> Self {
        Self {
            symbol_table: Default::default(),
            sources: Default::default(),
            stack: Default::default(),
            output: Stdout::new(),
            exporters: Default::default(),
            importers: Default::default(),
            diag: Default::default(),
        }
    }
}

impl Lookup<EvalError> for EvalContext {
    fn lookup(&self, name: &QualifiedName) -> EvalResult<Symbol> {
        log::debug!("Lookup symbol '{name:?}' (at line {:?}):", name.src_ref());

        log::trace!("- lookups -------------------------------------------------------");
        // collect all symbols that can be found and remember origin
        let result = [
            ("local", {
                match self.stack.lookup(name) {
                    Ok(SymbolOrName::Name(name)) => Ok(self.symbol_table.lookup(&name)?),
                    Ok(SymbolOrName::Symbol(symbol)) => Ok(symbol),
                    Err(err) => Err(err),
                }
            }),
            ("module", {
                self.lookup_within(name, self.stack.current_module_name())
            }),
            ("property", { self.lookup_property(name) }),
            ("workbench", {
                self.lookup_workbench(name).map_err(|err| err.into())
            }),
            ("global", {
                self.symbol_table.lookup(name).map_err(|err| err.into())
            }),
        ]
        .into_iter();

        log::trace!("- lookup result -------------------------------------------------");
        let mut errors = Vec::new();

        // collect ok-results and ambiguity errors
        let (found, mut ambiguities) = result.fold(
            (vec![], vec![]),
            |(mut oks, mut ambiguities), (origin, r)| {
                match r {
                    Ok(symbol) => oks.push((origin, symbol)),
                    Err(EvalError::AmbiguousSymbol { ambiguous, others }) => {
                        ambiguities.push((origin, EvalError::AmbiguousSymbol { ambiguous, others }))
                    }
                    Err(
                        // ignore all kinds of "not found" errors
                        EvalError::SymbolNotFound(_)
                        // for locals
                        | EvalError::LocalNotFound(_)
                        // for model proper
                        | EvalError::NoModelInWorkbench
                        | EvalError::PropertyNotFound(_)
                        | EvalError::NoPropertyId(_)
                        // for symbol table
                        | EvalError::ResolveError(ResolveError::SymbolNotFound(_))
                        | EvalError::ResolveError(ResolveError::ExternalPathNotFound(_))
                        | EvalError::ResolveError(ResolveError::NulHash),
                    ) => (),
                    Err(err) => errors.push((origin, err)),
                }
                (oks, ambiguities)
            },
        );

        // log any unexpected errors and return early
        if !errors.is_empty() {
            log::error!("Unexpected errors while lookup symbol '{name:?}':");
            errors
                .iter()
                .for_each(|(origin, err)| log::error!("Lookup ({origin}) error: {err}"));

            return Err(errors.remove(0).1);
        }

        // early emit any ambiguity error
        if !ambiguities.is_empty() {
            log::debug!(
                "{ambiguous} Symbol '{name:?}':\n{}",
                ambiguities
                    .iter()
                    .map(|(origin, err)| format!("{origin}: {err}"))
                    .collect::<Vec<_>>()
                    .join("\n"),
                ambiguous = crate::mark!(AMBIGUOUS)
            );
            return Err(ambiguities.remove(0).1);
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
            Some((origin, symbol)) => {
                // check if all findings point to the same symbol
                if !found.iter().all(|(_, x)| x == symbol) {
                    let origin = found
                        .iter()
                        .map(|(id, _)| *id)
                        .collect::<Vec<_>>()
                        .join(", ");
                    log::debug!(
                        "{ambiguous} symbol '{name:?}' in {origin}:\n{self:?}",
                        ambiguous = crate::mark!(AMBIGUOUS),
                        origin = found
                            .iter()
                            .map(|(id, _)| *id)
                            .collect::<Vec<_>>()
                            .join(" and ")
                    );
                    Err(EvalError::AmbiguousSymbol {
                        ambiguous: name.clone(),
                        others: origin,
                    })
                } else {
                    log::debug!(
                        "{found} symbol '{name:?}' in {origin}",
                        found = crate::mark!(FOUND)
                    );
                    symbol.set_use();
                    Ok(symbol.clone())
                }
            }
            None => {
                log::debug!(
                    "{not_found} Symbol '{name:?}'",
                    not_found = crate::mark!(NOT_FOUND)
                );

                Err(EvalError::SymbolNotFound(name.clone()))
            }
        }
    }
}

impl Diag for EvalContext {
    fn fmt_diagnosis(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result {
        self.diag.pretty_print(f, self)
    }

    fn warning_count(&self) -> u32 {
        self.diag.warning_count()
    }

    fn error_count(&self) -> u32 {
        self.diag.error_count()
    }

    fn error_lines(&self) -> std::collections::HashSet<usize> {
        self.diag.error_lines()
    }

    fn warning_lines(&self) -> std::collections::HashSet<usize> {
        self.diag.warning_lines()
    }
}

impl PushDiag for EvalContext {
    fn push_diag(&mut self, diag: Diagnostic) -> DiagResult<()> {
        let result = self.diag.push_diag(diag);
        log::trace!("Error Context:\n{self:?}");
        result
    }
}

impl GetSourceByHash for EvalContext {
    fn get_by_hash(&self, hash: u64) -> ResolveResult<Rc<SourceFile>> {
        self.sources.get_by_hash(hash)
    }
}

impl std::fmt::Debug for EvalContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(model) = self.get_model() {
            write!(f, "\nModel:\n")?;
            model.tree_print(f, TreeState::new_debug(4))?;
        }
        writeln!(f, "\nCurrent: {:?}", self.stack.current_name())?;
        writeln!(f, "\nModule: {:?}", self.stack.current_module_name())?;
        write!(f, "\nLocals Stack:\n{:?}", self.stack)?;
        writeln!(f, "\nCall Stack:")?;
        self.stack.pretty_print_call_trace(f, &self.sources)?;

        writeln!(f, "\nSymbol Table:")?;
        if self.has_errors() {
            writeln!(f, "{}\nErrors:", self.symbol_table)?;
            self.fmt_diagnosis(f)?;
        } else {
            write!(f, "{}", self.symbol_table)?;
        }
        Ok(())
    }
}

impl ImporterRegistryAccess for EvalContext {
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

impl ExporterAccess for EvalContext {
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
