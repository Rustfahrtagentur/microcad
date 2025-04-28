// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{diag::*, eval::*, rc::*, resolve::*, syntax::*, Id};
use log::*;

/// Context for evaluation
///
/// The context is used to store the current state of the evaluation.
/// A context is essentially a pile of symbol tables
pub struct EvalContext {
    /// root symbol
    root: SymbolNodeRcMut,
    /// List of all global symbols
    symbols: SymbolMap,
    /// Stack of currently opened scopes with local symbols while evaluation
    local_stack: LocalStack,
    /// Source file cache containing all source files loaded in the context and their syntax trees
    source_cache: SourceCache,
    /// Source file diagnostics
    diag_handler: DiagHandler,
    /// Output channel for __builtin::print
    output: Option<Output>,
}

impl EvalContext {
    /// Create a new context from a source file
    pub fn new(
        symbol: SymbolNodeRcMut,
        builtin: SymbolNodeRcMut,
        search_paths: &[std::path::PathBuf],
        output: Option<Output>,
    ) -> Self {
        debug!(
            "Creating Context (search paths: {})",
            search_paths
                .iter()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>()
                .join(",")
        );

        // if node owns a source file store this in the file cache
        let (source_cache, root) = match &symbol.borrow().def {
            SymbolDefinition::SourceFile(source_file) => (
                SourceCache::new(source_file.clone(), search_paths),
                symbol.clone(),
            ),
            _ => unreachable!("missing root source file"),
        };

        // prepare symbol map
        let mut symbols = SymbolMap::new();

        symbols.insert_node(builtin);

        // create namespaces for all files in search paths into symbol map
        let namespaces = source_cache.create_namespaces();
        namespaces.iter().for_each(|(_, namespace)| {
            symbols.insert_node(namespace.clone());
        });

        // insert root file into symbol map
        symbols.insert_node(root.clone());
        trace!("Symbols:\n{symbols}");

        // put all together
        Self {
            root,
            source_cache,
            symbols,
            diag_handler: Default::default(),
            local_stack: Default::default(),
            output,
        }
    }

    /// Create a new context from a source file
    pub fn from_source_file(
        source_file: Rc<SourceFile>,
        builtin: SymbolNodeRcMut,
        search_paths: &[std::path::PathBuf],
    ) -> Self {
        Self::from_source_file_with_output(source_file, builtin, search_paths, None)
    }

    /// Create a new context from a source file
    pub fn from_source_file_with_output(
        source_file: Rc<SourceFile>,
        builtin: SymbolNodeRcMut,
        search_paths: &[std::path::PathBuf],
        output: Option<Output>,
    ) -> Self {
        Self::new(
            SymbolNode::new(SymbolDefinition::SourceFile(source_file), None),
            builtin,
            search_paths,
            output,
        )
    }

    /// Add a named local value to current locals
    /// TODO: Is this special function really needed?
    pub fn add_local_value(&mut self, id: Id, value: Value) {
        self.local_stack
            .add(None, SymbolNode::new_constant(id, value));
    }

    /// Open a new scope
    pub fn open_scope(&mut self) {
        self.local_stack.open_scope();
    }

    /// Remove all local variables in the current scope and close it
    pub fn close_scope(&mut self) {
        self.local_stack.close_scope();
    }

    /// fetch global symbol from symbol map
    #[cfg(test)]
    pub fn fetch_global(&self, qualified_name: &QualifiedName) -> EvalResult<SymbolNodeRcMut> {
        self.symbols.search(&qualified_name.clone())
    }

    /// fetch local variable from local stack
    #[cfg(test)]
    pub fn fetch_local(&self, id: &Id) -> EvalResult<SymbolNodeRcMut> {
        self.local_stack.fetch(id)
    }

    /// fetch a value from local stack
    pub fn fetch_value(&self, name: &QualifiedName) -> EvalResult<Value> {
        if let Some(identifier) = name.single_identifier() {
            if let Ok(symbol) = self.local_stack.fetch(identifier.id()) {
                if let SymbolDefinition::Constant(_, value) = &symbol.borrow().def {
                    debug!("Fetching local value {name}");
                    return Ok(value.clone());
                }
            }
        }
        match &self.symbols.search(name)?.borrow().def {
            SymbolDefinition::Constant(_, value) => {
                debug!("Fetching global value {name}");
                Ok(value.clone())
            }

            _ => Err(EvalError::SymbolIsNotAValue(name.clone())),
        }
    }

    /// Find a symbol in the symbol table and copy it to the locals.
    /// Might load any related external file if not already loaded.
    /// - `name`: Name of the symbol to search for
    /// - `id`: if given overwrites the ID from qualified name (use as)
    pub fn use_symbol(
        &mut self,
        name: &QualifiedName,
        id: Option<Id>,
    ) -> EvalResult<SymbolNodeRcMut> {
        debug!("Using symbol {name}");
        let symbol = match self.symbols.search(name) {
            Ok(symbol) => symbol.clone(),
            _ => self.load_symbol(name)?,
        };
        self.local_stack.add(id, symbol.clone());
        trace!("Local Stack:\n{}", self.local_stack);
        Ok(symbol)
    }

    /// Find a symbol in the symbol table and copy all it's children to the locals.
    /// Might load any related external file if not already loaded.
    /// - `name`: Name of the symbol to search for
    pub fn use_symbols_of(&mut self, name: &QualifiedName) -> EvalResult<SymbolNodeRcMut> {
        debug!("Using all symbols in {name}");
        let symbol = match self.symbols.search(name) {
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
            Err(EvalError::NoSymbolFound(symbol.borrow().full_name()))
        } else {
            for (id, symbol) in symbol.borrow().children.iter() {
                self.local_stack.add(Some(id.clone()), symbol.clone());
            }
            trace!("Local Stack:\n{}", self.local_stack);
            Ok(symbol)
        }
    }

    /// lookup a symbol from a qualified name
    /// (might load any related external file if not already loaded)
    fn load_symbol(&mut self, name: &QualifiedName) -> EvalResult<SymbolNodeRcMut> {
        debug!("loading symbol {name}");

        // if symbol could not be found in symbol tree, try to load it from external file
        match self.source_cache.get_by_name(name) {
            Err(EvalError::SymbolMustBeLoaded(_, path)) => {
                // load source file
                let source_file = SourceFile::load(path.clone())?;
                // add to source cache
                let source_name = self.source_cache.insert(source_file.clone())?;
                // resolve source file
                let node = source_file.resolve(None);
                // search namespace to place loaded source file into
                let target = self.symbols.search(&source_name)?;
                // copy children into target namespace
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
        self.symbols.search(name)
    }

    /// Lookup for local or global symbol
    ///
    /// - looks in local stack
    /// - looks inm symbol map
    /// - follows aliases (use statements)
    /// - detect any ambiguity
    pub fn lookup(&self, name: &QualifiedName) -> EvalResult<SymbolNodeRcMut> {
        debug!("Lookup {name}");
        let local = if let Some(id) = name.single_identifier() {
            self.local_stack.fetch(id.id())
        } else {
            // split name
            let (id, mut name_rest) = name.split_first();
            // find a local by split id
            if let Ok(local) = self.local_stack.fetch(id.id()) {
                // get original name from the local symbol
                let mut alias_name = local.borrow().full_name();
                // concat split name rest to new namespace name
                alias_name.append(&mut name_rest);
                // lookup this new name
                self.lookup(&alias_name)
            } else {
                Err(EvalError::SymbolNotFound(name.clone()))
            }
        };
        // search for global symbol too
        let global = self.symbols.search(name);

        match (local, global) {
            (Ok(local), Ok(global)) => Err(EvalError::AmbiguousSymbol {
                ambiguous: name.clone(),
                local: local.borrow().full_name(),
                global: global.borrow().full_name(),
            }),
            (Ok(symbol), Err(_)) | (Err(_), Ok(symbol)) => {
                let def = &symbol.borrow().def;
                if let SymbolDefinition::Alias(_, name) = def {
                    trace!("Found alias => {name}");
                    return self.lookup(name);
                }
                Ok(symbol.clone())
            }
            (Err(_), Err(_)) => Err(EvalError::SymbolNotFound(name.clone())),
        }
    }

    /// Access diagnostic handler
    pub fn diag_handler(&self) -> &DiagHandler {
        &self.diag_handler
    }

    /// Access captured output
    pub fn output(&self) -> Option<String> {
        self.output
            .as_ref()
            .map(|output| output.get().expect("UTF8 error"))
    }

    /// return all occurred errors as string
    pub fn errors_as_str(&self) -> Option<String> {
        if self.diag_handler().has_errors() {
            Some(
                self.diag_handler()
                    .pretty_print_to_string(self)
                    .expect("cannot write into string"),
            )
        } else {
            None
        }
    }

    /// Print for __builtin::print
    pub fn print(&mut self, what: String) {
        if let Some(output) = &mut self.output {
            output.print(what).expect("could not write to output");
        } else {
            println!("{what}");
        }
    }

    /// get source code location of a src referrer
    pub fn locate(&self, referrer: &impl SrcReferrer) -> EvalResult<String> {
        Ok(format!(
            "{}:{}",
            self.get_by_hash(referrer.src_ref().source_hash())?
                .filename_as_str(),
            referrer.src_ref()
        ))
    }

    /// evaluate context to a value
    pub fn eval(&mut self) -> EvalResult<Value> {
        let source_file = match &self.root.borrow().def {
            SymbolDefinition::SourceFile(source_file) => source_file.clone(),
            _ => todo!(),
        };

        source_file.eval(self)
    }
}

impl PushDiag for EvalContext {
    fn push_diag(&mut self, diag: Diag) -> EvalResult<()> {
        self.diag_handler.push_diag(diag)
    }
}

impl GetSourceByHash for EvalContext {
    fn get_by_hash(&self, hash: u64) -> EvalResult<Rc<SourceFile>> {
        self.source_cache.get_by_hash(hash)
    }
}

impl std::fmt::Display for EvalContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Loaded files:\n{}\nLocals:\n{}\nSymbols:\n{}",
            self.source_cache, self.local_stack, self.symbols
        )
    }
}
