// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model::*, rc::*, resolve::*, syntax::*};

/// *Symbol table* holding global and local symbols.
///
/// The symbol table consists of the following members:
///
/// - One *root [`Symbol`]* resolved from the *initial source file*.
/// - A [`SourceCache`] of all *loaded source files* (accessible by *qualified name*, *file path* and *source hash*).
/// - A [`Stack`] of [`StackFrame`]s.
/// - A [`SymbolMap`] of all *global symbols*.
///
/// All these internal structures can be accessed by several implemented traits.
#[derive(Default)]
pub struct SymbolTable {
    sources: Sources,
    /// Symbol of the initial source file.
    pub root: Symbol,
    /// Stack of currently opened scopes with symbols while evaluation.
    pub stack: Stack,
    /// Global symbols (including root).
    pub symbols: SymbolMap,
    /// Source file diagnostics.
    pub diag_handler: DiagHandler,
}

impl SymbolTable {
    /// Root symbol (symbol node of initially read source file)
    /// List of all global symbols.
    /// Stack of currently opened scopes with local symbols while evaluation.
    /// Source file cache containing all source files loaded in the context and their syntax trees.
    pub fn new(root: Identifier, symbols: SymbolMap, sources: Sources) -> ResolveResult<Self> {
        // prepare symbol map

        let symbol_table = Self {
            sources,
            root: symbols.search(&QualifiedName::from_id(root))?,
            stack: Default::default(),
            symbols,
            diag_handler: Default::default(),
        };
        log::trace!("Initial symbol table:\n{symbol_table}");
        Ok(symbol_table)
    }

    /// Fetch local variable from local stack (for testing only).
    #[cfg(test)]
    pub fn fetch_local(&self, id: &Identifier) -> EvalResult<Symbol> {
        self.stack.fetch(id)
    }

    /// Lookup a symbol from global symbols.
    pub fn lookup_global(&mut self, name: &QualifiedName) -> ResolveResult<Symbol> {
        log::trace!("Looking for global symbol '{name:?}'");
        let symbol = match self.symbols.search(name) {
            Ok(symbol) => symbol.clone(),
            Err(err) => return Err(err)?,
        };
        log::trace!(
            "{found} global symbol: {symbol}",
            found = crate::mark!(FOUND),
        );
        Ok(symbol)
    }

    /// Lookup a symbol from local stack.
    fn lookup_local(&mut self, name: &QualifiedName) -> EvalResult<Symbol> {
        log::trace!("Looking for local symbol '{name:?}'");
        let symbol = if let Some(id) = name.single_identifier() {
            self.stack.fetch(id)
        } else {
            let (id, mut tail) = name.split_first();
            let local = self.stack.fetch(&id)?;
            let mut alias = local.full_name();
            alias.append(&mut tail);
            log::trace!("Following alias {alias}");
            self.lookup(&alias)
        };

        match symbol {
            Ok(symbol) => {
                log::trace!(
                    "{found} local symbol: {symbol}",
                    found = crate::mark!(FOUND),
                );
                Ok(symbol)
            }
            Err(err) => Err(err),
        }
    }

    fn lookup_within(&mut self, what: &QualifiedName, within: QualifiedName) -> EvalResult<Symbol> {
        log::trace!("Looking for symbol '{what:?}' within '{within:?}':",);

        // process internal supers
        let (what, within) = what.dissolve_super(within);

        let parents = self.symbols.path_to(&within)?;
        for (n, parent) in parents.iter().rev().enumerate() {
            log::trace!("  Looking in: {:?} for {:?}", parent.full_name(), what);
            if let Some(symbol) = parent.search(&what) {
                let alias = self.follow_alias(&symbol)?;
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

    fn lookup_workbench(&mut self, name: &QualifiedName) -> EvalResult<Symbol> {
        if let Some(workbench) = &self.stack.current_workbench_name() {
            log::trace!("Looking for symbol '{name:?}' in current workbench '{workbench:?}'");
            let name = &name.with_prefix(workbench);
            match self.lookup_global(name) {
                Ok(symbol) => {
                    if symbol.full_name() == *name {
                        log::trace!(
                            "{found} symbol in current module: {symbol}",
                            found = crate::mark!(FOUND),
                        );
                        return self.follow_alias(&symbol);
                    }
                }
                Err(err) => return Err(err)?,
            };
        }
        Err(EvalError::SymbolNotFound(name.clone()))
    }

    fn de_alias(&mut self, name: &QualifiedName) -> QualifiedName {
        (1..name.len())
            .rev()
            .filter_map(|p| {
                if let Ok(symbol) = self.lookup_global(&QualifiedName::no_ref(name[0..p].to_vec()))
                {
                    Some((p, symbol))
                } else {
                    None
                }
            })
            .find_map(|(p, symbol)| {
                symbol.with_def(|def| {
                    if let SymbolDefinition::Alias(.., alias) = def {
                        let suffix: QualifiedName = name[p..].iter().cloned().collect();
                        let new_name = suffix.with_prefix(alias);
                        log::trace!("De-aliased name: {name:?} into {new_name:?}");
                        Some(new_name)
                    } else {
                        None
                    }
                })
            })
            .unwrap_or(name.clone())
    }

    fn follow_alias(&mut self, symbol: &Symbol) -> EvalResult<Symbol> {
        // execute alias from any use statement
        symbol.with_def(|def| {
            if let SymbolDefinition::Alias(.., name) = def {
                log::trace!("{found} alias => {name:?}", found = crate::mark!(FOUND));
                Ok(self.lookup(name)?)
            } else {
                Ok(symbol.clone())
            }
        })
    }

    /// Return search paths of this symbol table.
    pub fn search_paths(&self) -> &Vec<std::path::PathBuf> {
        self.sources.search_paths()
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

impl Lookup for SymbolTable {
    fn lookup(&mut self, name: &QualifiedName) -> EvalResult<Symbol> {
        log::debug!("Lookup symbol '{name:?}' (at line {:?}):", name.src_ref());

        let name = &self.de_alias(name);

        log::trace!("- lookups -------------------------------------------------------");
        // collect all symbols that can be found and remember origin
        let result = [
            ("local", self.lookup_local(name)),
            (
                "module",
                self.lookup_within(name, self.stack.current_module_name()),
            ),
            ("workbench", self.lookup_workbench(name)),
            ("global", self.lookup_global(name).map_err(|e| e.into())),
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
                if let Ok(symbol) = self.follow_alias(&symbol) {
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
                if !found.iter().all(|(_, x)| x == first) {
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

impl Locals for SymbolTable {
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

impl UseSymbol for SymbolTable {
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
                self.symbols.search(within)?.insert(id, symbol);
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
                let within = if within.is_empty() {
                    None
                } else {
                    Some(self.symbols.search(within)?)
                };
                for (id, symbol) in symbol.clone_children() {
                    let symbol = symbol.clone_with_visibility(visibility);
                    if let Some(within) = &within {
                        within.insert(id.clone(), symbol);
                    } else {
                        self.symbols.insert(id.clone(), symbol);
                    }
                }

                log::trace!("Symbol Table:\n{}", self.symbols);
            }

            if self.is_code() {
                for (id, symbol) in symbol.clone_children() {
                    self.stack.put_local(Some(id), symbol)?;
                }
                log::trace!("Local Stack:\n{}", self.stack);
            }
            Ok(symbol)
        }
    }
}

impl std::fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nLoaded files:\n{}", self.sources)?;
        writeln!(f, "\nCurrent: {}", self.stack.current_name())?;
        writeln!(f, "\nModule: {}", self.stack.current_module_name())?;
        write!(f, "\nLocals Stack:\n{}", self.stack)?;
        writeln!(f, "\nCall Stack:")?;
        self.stack.pretty_print_call_trace(f, &self.sources)?;
        writeln!(f, "\nSymbols:\n{}", self.symbols)
    }
}

impl GetSourceByHash for SymbolTable {
    fn get_by_hash(&self, hash: u64) -> ResolveResult<std::rc::Rc<SourceFile>> {
        self.sources.get_by_hash(hash)
    }
}
