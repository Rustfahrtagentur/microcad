// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, rc::*, resolve::*, syntax::*};

/// *Symbol table* holding global and local symbols.
///
/// A the symbol table consists of the following members:
///
/// - One *root symbol* resolved from the initially read source file.
/// - A map of all *global symbols*.
/// - A stack of local scope frames that store *local values* and *local aliases* from use statements.
/// - A map of all *loaded source files* (accessible by name, path and hash).
///
/// All these internal structures can be accessed by several implemented traits.
pub struct SymbolTable {
    /// Root symbol (symbol node of initially read source file)
    pub root: Symbol,
    /// List of all global symbols.
    globals: SymbolMap,
    /// Stack of currently opened scopes with symbols while evaluation.
    stack: Stack,
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
            stack: Default::default(),
            cache: source_cache,
        };
        log::trace!("Initial context:\n{context}");
        context
    }

    /// Fetch global symbol from symbol map (for testing only).
    #[cfg(test)]
    pub fn fetch_global(&self, qualified_name: &QualifiedName) -> EvalResult<Symbol> {
        self.globals.search(&qualified_name.clone())
    }

    /// Fetch local variable from local stack (for testing only).
    #[cfg(test)]
    pub fn fetch_local(&self, id: &Identifier) -> EvalResult<Symbol> {
        self.stack.fetch(id)
    }

    /// lookup a symbol from local stack
    fn lookup_local(&mut self, name: &QualifiedName) -> EvalResult<Symbol> {
        if let Some(id) = name.single_identifier() {
            self.stack.fetch(id)
        } else {
            let (id, mut tail) = name.split_first();
            if let Ok(local) = self.stack.fetch(&id) {
                let mut alias = local.full_name();
                alias.append(&mut tail);
                log::trace!("Following alias {alias}");
                self.lookup(&alias)
            } else {
                Err(EvalError::SymbolNotFound(name.clone()))
            }
        }
    }

    /// lookup a symbol from global symbols
    fn lookup_global(&mut self, name: &QualifiedName) -> EvalResult<Symbol> {
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
    fn load_symbol(&mut self, name: &QualifiedName) -> EvalResult<Symbol> {
        log::debug!("loading symbol {name}");

        // if symbol could not be found in symbol tree, try to load it from external file
        match self.cache.get_by_name(name) {
            Err(EvalError::SymbolMustBeLoaded(_, path)) => {
                let source_file = SourceFile::load(path.clone())?;
                let source_name = self.cache.insert(source_file.clone())?;
                let node = source_file.resolve(None);
                // search namespace to place loaded source file into
                let target = self.globals.search(&source_name)?;
                Symbol::move_children(&target, &node);
                // mark target as "loaded" by changing the SymbolDefinition type
                target.external_to_namespace();
            }
            Ok(_) => (),
            _ => {
                return Err(EvalError::SymbolNotFound(name.clone()));
            }
        }

        // get symbol from symbol map
        self.globals.search(name)
    }

    fn follow_alias(&mut self, symbol: Symbol) -> EvalResult<Symbol> {
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

impl Lookup for SymbolTable {
    fn lookup(&mut self, name: &QualifiedName) -> EvalResult<Symbol> {
        log::debug!("Lookup {name}");

        // collect all symbols that can be found
        let result = [self.lookup_local(name), self.lookup_global(name)].into_iter();

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

    fn get_local_value(&mut self, id: &Identifier) -> EvalResult<Value> {
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
}

impl UseSymbol for SymbolTable {
    fn use_symbol(&mut self, name: &QualifiedName, id: Option<Identifier>) -> EvalResult<Symbol> {
        log::debug!("Using symbol {name}");

        let symbol = self.lookup(name)?;
        self.stack.put_local(id, symbol.clone())?;
        log::trace!("Local Stack:\n{}", self.stack);

        Ok(symbol)
    }

    fn use_symbols_of(&mut self, name: &QualifiedName) -> EvalResult<Symbol> {
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
            }
            log::trace!("Local Stack:\n{}", self.stack);
            Ok(symbol)
        }
    }
}

impl std::fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nLoaded files:\n{}", self.cache)?;
        writeln!(f, "\nNamespace: {}", self.stack.current_namespace())?;
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
