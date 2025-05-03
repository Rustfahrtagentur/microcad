// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, resolve::*, syntax::*};

/// Symbol table holding global and local symbols
pub struct SymbolTable {
    /// Root symbol (symbol node of initially read source file)
    pub root: SymbolNodeRcMut,
    /// List of all global symbols.
    globals: SymbolMap,
    /// Stack of currently opened scopes with local symbols while evaluation.
    locals: LocalStack,
    /// Source file cache containing all source files loaded in the context and their syntax trees.
    cache: SourceCache,
}

impl SymbolTable {
    /// Root symbol (symbol node of initially read source file)
    /// List of all global symbols.
    /// Stack of currently opened scopes with local symbols while evaluation.
    /// Source file cache containing all source files loaded in the context and their syntax trees.
    pub fn new(
        root: SymbolNodeRcMut,
        builtin: SymbolNodeRcMut,
        search_paths: &[std::path::PathBuf],
    ) -> Self {
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

        // create namespaces for all files in search paths into symbol map
        source_cache
            .create_namespaces()
            .iter()
            .for_each(|(id, namespace)| {
                globals.insert_node(id.clone(), namespace.clone());
            });

        let context = Self {
            root,
            globals,
            locals: Default::default(),
            cache: source_cache,
        };
        log::trace!("Initial context:\n{context}");
        context
    }

    /// Fetch global symbol from symbol map (for testing only).
    #[cfg(test)]
    pub fn fetch_global(&self, qualified_name: &QualifiedName) -> EvalResult<SymbolNodeRcMut> {
        self.globals.search(&qualified_name.clone())
    }

    /// Fetch local variable from local stack (for testing only).
    #[cfg(test)]
    pub fn fetch_local(&self, id: &Identifier) -> EvalResult<SymbolNodeRcMut> {
        self.locals.fetch(id)
    }

    /// lookup a symbol from local stack
    fn lookup_local(&mut self, name: &QualifiedName) -> EvalResult<SymbolNodeRcMut> {
        if let Some(id) = name.single_identifier() {
            self.locals.fetch(id)
        } else {
            let (id, mut tail) = name.split_first();
            if let Ok(local) = self.locals.fetch(&id) {
                let mut alias = local.borrow().full_name();
                alias.append(&mut tail);
                log::trace!("Following alias {alias}");
                self.lookup(&alias)
            } else {
                Err(EvalError::SymbolNotFound(name.clone()))
            }
        }
    }

    /// lookup a symbol from global symbols
    fn lookup_global(&mut self, name: &QualifiedName) -> EvalResult<SymbolNodeRcMut> {
        let symbol = match self.globals.search(name) {
            Ok(symbol) => symbol.clone(),
            _ => self.load_symbol(name)?,
        };
        Ok(symbol)
    }

    /// Lookup a symbol from a qualified name.
    ///
    /// Might load any related external file if not already loaded.
    ///
    /// # Arguments
    /// - `name`: Name of the symbol to load
    fn load_symbol(&mut self, name: &QualifiedName) -> EvalResult<SymbolNodeRcMut> {
        log::debug!("loading symbol {name}");

        // if symbol could not be found in symbol tree, try to load it from external file
        match self.cache.get_by_name(name) {
            Err(EvalError::SymbolMustBeLoaded(_, path)) => {
                let source_file = SourceFile::load(path.clone())?;
                let source_name = self.cache.insert(source_file.clone())?;
                let node = source_file.resolve(None);
                // search namespace to place loaded source file into
                let target = self.globals.search(&source_name)?;
                SymbolNode::move_children(&target, &node);
                // mark target as "loaded" by changing the SymbolDefinition type
                target.borrow_mut().external_to_namespace();
            }
            Ok(_) => (),
            _ => {
                return Err(EvalError::SymbolNotFound(name.clone()));
            }
        }

        // get symbol from symbol map
        self.globals.search(name)
    }

    fn follow_alias(&mut self, symbol: SymbolNodeRcMut) -> EvalResult<SymbolNodeRcMut> {
        // execute alias from any use statement
        let def = &symbol.borrow().def;
        if let SymbolDefinition::Alias(_, name) = def {
            log::trace!("Found alias => {name}");
            self.lookup(name)
        } else {
            Ok(symbol.clone())
        }
    }
}

impl Symbols for SymbolTable {
    fn lookup(&mut self, name: &QualifiedName) -> EvalResult<SymbolNodeRcMut> {
        log::debug!("Lookup {name}");

        // collect all symbols that can be found
        let result = [
            self.lookup_local(name),
            self.lookup_global(name),
            self.lookup_global(&name.with_prefix(&self.locals.current_namespace())),
        ]
        .into_iter();

        // collect ok-results and ambiguity errors
        let (found, mut ambiguous) =
            result.fold((Vec::new(), Vec::new()), |(mut oks, mut ambiguity), r| {
                match r {
                    Ok(symbol) => oks.push(symbol),
                    Err(EvalError::AmbiguousSymbol { ambiguous, others }) => {
                        ambiguity.push(EvalError::AmbiguousSymbol { ambiguous, others })
                    }

                    Err(_) => (),
                }
                (oks, ambiguity)
            });

        // early emit any ambiguity error
        if !ambiguous.is_empty() {
            return Err(ambiguous.remove(0));
        }

        // follow aliases
        let found: Vec<SymbolNodeRcMut> = found
            .into_iter()
            .map(|symbol| self.follow_alias(symbol))
            .filter_map(Result::ok)
            .collect();

        // check for ambiguity in what's left
        match found.first() {
            Some(first) => {
                // check if all findings point to the same symbol
                if !found.iter().all(|x| std::rc::Rc::ptr_eq(x, first)) {
                    Err(EvalError::AmbiguousSymbol {
                        ambiguous: name.clone(),
                        others: found.iter().cloned().collect::<SymbolNodes>(),
                    })
                } else {
                    Ok(first.clone())
                }
            }
            None => Err(EvalError::SymbolNotFound(name.clone())),
        }
    }
}

impl Locals for SymbolTable {
    fn open_source(&mut self, id: Identifier) {
        self.locals.open_source(id);
    }

    fn open_namespace(&mut self, id: Identifier) {
        self.locals.open_namespace(id);
        log::trace!("open namespace -> {}", self.locals.current_namespace());
    }

    fn open_module(&mut self, id: Identifier) {
        self.locals.open_module(id);
        log::trace!("closed namespace -> {}", self.locals.current_namespace());
    }

    fn open_scope(&mut self) {
        self.locals.open_scope();
    }

    fn close(&mut self) {
        self.locals.close();
        log::trace!("closed -> {}", self.locals.current_namespace());
    }

    fn add_local_value(&mut self, id: Identifier, value: Value) -> EvalResult<()> {
        self.locals.add_local_value(id, value)
    }

    fn fetch(&self, id: &Identifier) -> EvalResult<SymbolNodeRcMut> {
        self.locals.fetch(id)
    }
}

impl UseSymbol for SymbolTable {
    fn use_symbol(
        &mut self,
        name: &QualifiedName,
        id: Option<Identifier>,
    ) -> EvalResult<SymbolNodeRcMut> {
        log::debug!("Using symbol {name}");

        let symbol = self.lookup(name)?;
        self.locals.add(id, symbol.clone())?;
        log::trace!("Local Stack:\n{}", self.locals);

        Ok(symbol)
    }

    fn use_symbols_of(&mut self, name: &QualifiedName) -> EvalResult<SymbolNodeRcMut> {
        log::debug!("Using all symbols in {name}");

        let symbol = match self.lookup(name) {
            Ok(symbol) => {
                //  load external file if symbol was not loaded before
                let ext = symbol.borrow().is_external();
                match ext {
                    true => self.load_symbol(name)?,
                    false => symbol.clone(),
                }
            }
            _ => self.load_symbol(name)?,
        };

        if symbol.borrow().children.is_empty() {
            Err(EvalError::NoSymbolsFound(symbol.borrow().full_name()))
        } else {
            for (id, symbol) in symbol.borrow().children.iter() {
                self.locals.add(Some(id.clone()), symbol.clone())?;
            }
            log::trace!("Local Stack:\n{}", self.locals);
            Ok(symbol)
        }
    }
}

impl std::fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Loaded files:\n{files}\nLocals [{name}]:\n{locals}\nSymbols:\n{symbols}",
            files = self.cache,
            name = self.locals.current_namespace(),
            locals = self.locals,
            symbols = self.globals
        )
    }
}

impl GetSourceByHash for SymbolTable {
    fn get_by_hash(&self, hash: u64) -> EvalResult<std::rc::Rc<SourceFile>> {
        self.cache.get_by_hash(hash)
    }
}
