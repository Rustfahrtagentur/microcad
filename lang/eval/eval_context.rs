// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Id, diag::*, eval::*, rc_mut::*, resolve::*, syntax::*};
use log::*;

/// Context for evaluation
///
/// The context is used to store the current state of the evaluation.
/// A context is essentially a stack of symbol tables
pub struct EvalContext {
    /// Tree of all evaluated symbols
    symbols: SymbolMap,
    /// Current node while evaluation
    pub current: RcMut<SymbolNode>,
    /// Stack of currently opened scopes with local symbols while evaluation
    scope_stack: ScopeStack,
    /// Source file cache containing all source files loaded in the context
    source_cache: SourceCache,
    /// Source file diagnostics
    diag_handler: DiagHandler,
    /// Output channel for __builtin::print
    output: Option<Output>,
}

/// Look up result
pub enum LookUp {
    /// Look up failed
    NotFound(SrcRef),
    /// found local variable with given Id
    Local(Id),
    /// found global symbol with given qualified name
    Symbol(QualifiedName),
}

impl EvalContext {
    /// Create a new context from a source file
    pub fn new(
        symbol: RcMut<SymbolNode>,
        search_paths: Vec<std::path::PathBuf>,
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
        let (source_cache, current) = match &symbol.borrow().def {
            SymbolDefinition::SourceFile(source_file) => (
                SourceCache::new(source_file.clone(), search_paths),
                SymbolNode::new(SymbolDefinition::SourceFile(source_file.clone()), None),
            ),
            _ => unreachable!("missing root source file"),
        };

        let mut symbols = SymbolMap::new();
        let namespaces = source_cache.create_namespaces();
        namespaces.iter().for_each(|(_, namespace)| {
            symbols.insert(namespace.borrow().id(), namespace.clone());
        });

        symbols.insert(current.borrow().id(), current.clone());

        trace!("Symbols:\n{symbols:#?}");

        Self {
            source_cache,
            symbols,
            current,
            diag_handler: Default::default(),
            scope_stack: Default::default(),
            output,
        }
    }

    /// Create a new context from a source file
    pub fn from_source_file(
        source_file: Rc<SourceFile>,
        search_paths: Vec<std::path::PathBuf>,
    ) -> Self {
        Self::from_source_file_with_output(source_file, search_paths, None)
    }

    /// Create a new context from a source file
    pub fn from_source_file_with_output(
        source_file: Rc<SourceFile>,
        search_paths: Vec<std::path::PathBuf>,
        output: Option<Output>,
    ) -> Self {
        Self::new(
            SymbolNode::new(SymbolDefinition::SourceFile(source_file), None),
            search_paths,
            output,
        )
    }

    /// Add a local value
    pub fn add_local_value(&mut self, id: Id, value: Value) {
        self.scope_stack.add(id, LocalDefinition::Value(value));
    }

    /// Add symbol to current symbol table
    pub fn add_symbol(&mut self, symbol: RcMut<SymbolNode>) {
        self.symbols.insert(symbol.borrow().id(), symbol.clone());
    }

    /// Open a new scope
    pub fn open_scope(&mut self) {
        self.scope_stack.open_scope();
    }

    /// Remove all local variables in the current scope and close it
    pub fn close_scope(&mut self) {
        self.scope_stack.close_scope();
    }

    /// fetch symbol from symbol table
    pub fn fetch_symbol(&self, qualified_name: &QualifiedName) -> EvalResult<RcMut<SymbolNode>> {
        if let Some(child) = self.current.borrow().search_up(&qualified_name.clone()) {
            Ok(child)
        } else {
            Err(super::EvalError::SymbolNotFound(
                qualified_name.clone(),
                self.current.borrow().name()?,
            ))
        }
    }

    /// fetch local variable
    pub fn fetch_local<'a>(&'a self, id: &Id) -> EvalResult<&'a LocalDefinition> {
        if let Some(def) = self.scope_stack.fetch(id) {
            Ok(def)
        } else {
            Err(super::EvalError::LocalNotFound(id.clone()))
        }
    }

    /// fetch a value from a local variable or symbol table
    pub fn fetch_value(&self, name: &QualifiedName) -> EvalResult<Value> {
        if let Some(identifier) = name.single_identifier() {
            if let Ok(LocalDefinition::Value(value)) = self.fetch_local(identifier.id()) {
                return Ok(value.clone());
            }
        }

        let symbol = self.fetch_symbol(name)?;

        match &symbol.borrow().def {
            SymbolDefinition::Constant(_, value) => Ok(value.clone()),
            _ => Err(EvalError::SymbolIsNotAValue(name.clone())),
        }
    }

    /// Find a symbol in the symbol table and add it at the currently processed node
    pub fn use_symbol(&mut self, name: &QualifiedName) -> EvalResult<()> {
        debug!("using symbol {name} in {}", self.current.borrow().def.id());
        // search for name upwards in symbol tree
        if let Some(child) = self.current.borrow().search_up(name) {
            SymbolNode::insert_child(&self.current, child);
            return Ok(());
        }
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
                target.borrow_mut().copy_children(node);
            }
            Ok(_) => (),
            _ => {
                return Err(EvalError::SymbolNotFound(
                    name.clone(),
                    self.current.borrow().name()?,
                ));
            }
        }

        let symbol = self.symbols.search(name)?;
        // insert node into symbols
        self.current.borrow_mut().insert(name, symbol.clone());

        trace!("Symbols:\n{}", self.symbols);

        Ok(())
    }

    /// look up a symbol name in either local variables or symbol table
    pub fn look_up(&self, name: &QualifiedName) -> LookUp {
        let id: Result<Id, _> = name.clone().try_into();
        if let Ok(id) = id {
            if self.fetch_local(&id).is_ok() {
                return LookUp::Local(id);
            }
        }
        if self.fetch_symbol(name).is_ok() {
            return LookUp::Symbol(name.clone());
        }
        LookUp::NotFound(name.src_ref())
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

    /// Print for __builtin::print
    pub fn print(&mut self, what: String) {
        if let Some(output) = &mut self.output {
            output.print(what).expect("could not write to output");
        } else {
            println!("{what}");
        }
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
