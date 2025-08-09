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
    /// Symbol of the initial source file.
    pub root: Symbol,
    /// List of all global symbols.
    globals: SymbolMap,
    /// Stack of currently opened scopes with symbols while evaluation.
    pub stack: Stack,
    /// Source file cache containing all source files loaded in the context and their syntax trees.
    cache: SourceCache,
}

impl SymbolTable {
    /// Root symbol (symbol node of initially read source file)
    /// List of all global symbols.
    /// Stack of currently opened scopes with local symbols while evaluation.
    /// Source file cache containing all source files loaded in the context and their syntax trees.
    pub fn new(root: Symbol, builtin: Symbol, search_paths: &[std::path::PathBuf]) -> Self {
        // if node owns a source file store this in the file cache
        let (source_cache, root) = match &root.borrow().def {
            SymbolDefinition::SourceFile(source_file) => (
                SourceCache::new(source_file.clone(), search_paths),
                root.clone(),
            ),
            _ => unreachable!("missing root source file"),
        };

        // prepare symbol map
        let mut globals = root.borrow().children.clone().detach_from_parent();
        globals.add_node(builtin);

        // create modules for all files in search paths into symbol map
        source_cache
            .create_modules()
            .iter()
            .for_each(|(id, module)| {
                globals.insert_node(id.clone(), module.clone());
            });

        let context = Self {
            root,
            globals,
            stack: Default::default(),
            cache: source_cache,
        };
        log::trace!("Initial context:\n{context}");
        context
    }

    /// Fetch global symbol from symbol map (for testing only).
    #[cfg(test)]
    pub fn fetch_global(&self, qualified_name: &QualifiedName) -> EvalResult<Symbol> {
        self.globals.search(qualified_name)
    }

    /// Fetch local variable from local stack (for testing only).
    #[cfg(test)]
    pub fn fetch_local(&self, id: &Identifier) -> EvalResult<Symbol> {
        self.stack.fetch(id)
    }

    /// Lookup a symbol from local stack.
    fn lookup_local(&mut self, name: &QualifiedName) -> EvalResult<Symbol> {
        log::trace!("looking up locally for '{name}'");
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
                log::trace!("local found '{name}' = '{}'", symbol.full_name());
                Ok(symbol)
            }
            Err(err) => Err(err),
        }
    }

    /// Lookup a symbol from global symbols.
    fn lookup_global(&mut self, name: &QualifiedName) -> EvalResult<Symbol> {
        log::trace!("looking up globally for '{name}'");
        let symbol = match self.globals.search(name) {
            Ok(symbol) => symbol.clone(),
            Err(EvalError::SymbolNotFound(_)) => self.load_symbol(name)?,
            err => return err,
        };
        log::debug!("global found '{name}' = '{}'", symbol.full_name());
        Ok(symbol)
    }

    fn lookup_current(&mut self, name: &QualifiedName) -> EvalResult<Symbol> {
        let name = &name.with_prefix(&self.stack.current_module_name());
        log::trace!("lookup current in for '{name}'");
        match self.lookup_global(name) {
            Ok(symbol) => {
                if symbol.full_name() == *name {
                    log::debug!(
                        "lookup in current module found '{name}' = '{}'",
                        symbol.full_name()
                    );
                    return self.follow_alias(symbol);
                }
            }
            err => return err,
        };
        Err(EvalError::SymbolNotFound(name.clone()))
    }

    /// Lookup a symbol from global symbols but relatively to the file the given `name` came from.
    ///
    /// - `name`: *Qualified`name* to search for (must have a proper *source code reference*.
    ///
    /// Returns found symbol or error if `name` does not have a *source code reference* or symbol could not be found.
    fn lookup_relatively(&mut self, name: &QualifiedName) -> EvalResult<Symbol> {
        if name.src_ref().is_some() {
            let name =
                &name.with_prefix(self.cache.get_name_by_hash(name.src_ref().source_hash())?);
            log::trace!("lookup relatively for '{name}'");
            let symbol = self.lookup_global(name)?;
            log::debug!(
                "lookup relatively found '{name}' = '{}'",
                symbol.full_name()
            );
            return self.follow_alias(symbol);
        }
        Err(EvalError::SymbolNotFound(name.clone()))
    }

    fn de_alias(&mut self, name: &QualifiedName) -> QualifiedName {
        log::trace!("de-alias: {name}..");
        for p in (1..name.len()).rev() {
            if let Ok(symbol) = self.lookup_global(&QualifiedName::no_ref(name[0..p].to_vec())) {
                if let SymbolDefinition::Alias(_, alias) = &symbol.borrow().def {
                    let suffix: QualifiedName = name[p..].iter().cloned().collect();
                    let new_name = suffix.with_prefix(alias);
                    log::debug!("de-aliased: {name} into {new_name}");
                    return new_name;
                }
            }
        }
        log::trace!("not de-aliased: {name}..");
        name.clone()
    }

    /// Load a symbol from a qualified name.
    ///
    /// Might load any related external file if not already loaded.
    ///
    /// # Arguments
    /// - `name`: Name of the symbol to load
    fn load_symbol(&mut self, name: &QualifiedName) -> EvalResult<Symbol> {
        log::trace!("trying to load symbol {name}");

        // if symbol could not be found in symbol tree, try to load it from external file
        match self.cache.get_by_name(name) {
            Err(EvalError::SymbolMustBeLoaded(_, path)) => {
                log::debug!("loading symbol {name} from {path:?}");
                let source_file =
                    SourceFile::load_with_name(path.clone(), self.cache.name_by_path(&path)?)?;
                let source_name = self.cache.insert(source_file.clone())?;
                let node = source_file.resolve(None);
                // search module where to place loaded source file into
                let target = self.globals.search(&source_name)?;
                Symbol::move_children(&target, &node);
                // mark target as "loaded" by changing the SymbolDefinition type
                target.external_to_module();
            }
            Ok(_) => (),
            Err(EvalError::SymbolNotFound(_)) => {
                return Err(EvalError::SymbolNotFound(name.clone()))
            }
            Err(err) => return Err(err),
        }

        // get symbol from symbol map
        self.globals.search(name)
    }

    fn follow_alias(&mut self, symbol: Symbol) -> EvalResult<Symbol> {
        // execute alias from any use statement
        let def = &symbol.borrow().def;
        if let SymbolDefinition::Alias(_, name) = def {
            log::trace!("Found alias => {name}");
            Ok(self.lookup(name)?)
        } else {
            Ok(symbol.clone())
        }
    }

    /// Return search paths of this symbol table.
    pub fn search_paths(&self) -> &Vec<std::path::PathBuf> {
        self.cache.search_paths()
    }
}

impl Lookup for SymbolTable {
    fn lookup(&mut self, name: &QualifiedName) -> EvalResult<Symbol> {
        log::trace!("looking up {name}");

        let name = &self.de_alias(name);

        // collect all symbols that can be found
        let result = [
            self.lookup_local(name),
            self.lookup_current(name),
            self.lookup_global(name),
            self.lookup_relatively(name),
        ]
        .into_iter();

        let mut errors = Vec::new();

        // collect ok-results and ambiguity errors
        let (found, mut ambiguous) =
            result.fold((Vec::new(), Vec::new()), |(mut oks, mut ambiguity), r| {
                match r {
                    Ok(symbol) => oks.push(symbol),
                    Err(EvalError::AmbiguousSymbol { ambiguous, others }) => {
                        ambiguity.push(EvalError::AmbiguousSymbol { ambiguous, others })
                    }
                    Err(
                        EvalError::SymbolNotFound(_)
                        | EvalError::LocalNotFound(_)
                        | EvalError::ExternalPathNotFound(_)
                        | EvalError::NulHash,
                    ) => (),
                    Err(err) => errors.push(err),
                }
                (oks, ambiguity)
            });

        // log any unexpected errors and return early
        if !errors.is_empty() {
            errors
                .iter()
                .for_each(|err| log::error!("lookup error: {err}"));
            return Err(errors.remove(0));
        }

        // early emit any ambiguity error
        if !ambiguous.is_empty() {
            return Err(ambiguous.remove(0));
        }

        // follow aliases
        let found: Vec<Symbol> = found
            .into_iter()
            .map(|symbol| self.follow_alias(symbol))
            .filter_map(Result::ok)
            .collect();

        // check for ambiguity in what's left
        match found.first() {
            Some(first) => {
                // check if all findings point to the same symbol
                if !found.iter().all(|x| Rc::ptr_eq(x, first)) {
                    Err(EvalError::AmbiguousSymbol {
                        ambiguous: name.clone(),
                        others: found.into(),
                    })
                } else {
                    log::debug!("lookup of {name} successful");
                    Ok(first.clone())
                }
            }
            None => Err(EvalError::SymbolNotFound(name.clone())),
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
    fn use_symbol(&mut self, name: &QualifiedName, id: Option<Identifier>) -> EvalResult<Symbol> {
        log::debug!("Using symbol {name}");

        let symbol = self.lookup(name)?;
        self.stack.put_local(id, symbol.clone())?;
        log::trace!("Local Stack:\n{}", self.stack);

        Ok(symbol)
    }

    fn use_symbols_of(
        &mut self,
        name: &QualifiedName,
        within: &QualifiedName,
    ) -> EvalResult<Symbol> {
        log::debug!("Using all symbols in {name}");

        let symbol = match self.lookup(name) {
            Ok(symbol) => {
                //  load external file if symbol was not loaded before
                let ext = symbol.is_external();
                match ext {
                    true => self.load_symbol(name)?,
                    false => symbol.clone(),
                }
            }
            _ => self.load_symbol(name)?,
        };

        if symbol.is_empty() {
            Err(EvalError::NoSymbolsToUse(symbol.full_name()))
        } else {
            for (id, symbol) in symbol.borrow().children.iter() {
                self.stack.put_local(Some(id.clone()), symbol.clone())?;
                if within.is_empty() {
                    self.globals.insert(id.clone(), symbol.clone());
                } else {
                    self.globals
                        .search(within)?
                        .borrow_mut()
                        .children
                        .insert(id.clone(), symbol.clone());
                }
            }
            log::trace!("Local Stack:\n{}", self.stack);
            Ok(symbol)
        }
    }
}

impl std::fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nLoaded files:\n{}", self.cache)?;
        writeln!(f, "\nCurrent: {}", self.stack.current_name())?;
        writeln!(f, "\nModule: {}", self.stack.current_module_name())?;
        write!(f, "\nLocals Stack:\n{}", self.stack)?;
        writeln!(f, "\nCall Stack:")?;
        self.stack.pretty_print_call_trace(f, &self.cache)?;
        writeln!(f, "\nSymbols:\n{}", self.globals)
    }
}

impl GetSourceByHash for SymbolTable {
    fn get_by_hash(&self, hash: u64) -> EvalResult<std::rc::Rc<SourceFile>> {
        self.cache.get_by_hash(hash)
    }
}
