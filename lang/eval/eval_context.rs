// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use crate::{diag::*, resolve::*, source_file_cache::*, syntax::*};

/// Context for evaluation
///
/// The context is used to store the current state of the evaluation.
/// A context is essentially a stack of symbol tables
pub struct EvalContext {
    /// Tree of all evaluated symbols
    symbols: RcMut<SymbolNode>,
    /// Stack of currently opened scopes with local symbols while evaluation
    scope_stack: ScopeStack,
    /// Current source file being evaluated
    current_source_file: Option<Rc<SourceFile>>,
    /// Source file cache containing all source files loaded in the context
    source_files: SourceFileCache,
    /// Source file diagnostics
    diag_handler: DiagHandler,
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
    pub fn from_source_file(source_file: Rc<SourceFile>) -> Self {
        let mut ctx = Self {
            current_source_file: Some(source_file.clone()),
            symbols: SymbolNode::new(SymbolDefinition::SourceFile(source_file.clone()), None),
            source_files: Default::default(),
            diag_handler: Default::default(),
            scope_stack: Default::default(),
        };

        ctx.source_files.add(source_file);
        ctx
    }

    /// Return the current source file
    ///
    /// Note: This should not be an optional value, as the context is always created with a source file
    pub fn current_source_file(&self) -> Option<Rc<SourceFile>> {
        self.current_source_file.clone()
    }

    /// Add source file to Context
    pub fn add_source_file(&mut self, source_file: SourceFile) {
        self.source_files.add(Rc::new(source_file))
    }

    /// Add a local value
    pub fn add_local_value(&mut self, id: Id, value: Value) {
        self.scope_stack.add(id, LocalDefinition::Value(value));
    }

    /// Return reference to the symbols node which is currently processed
    pub fn current_node(&self) -> RcMut<SymbolNode> {
        self.symbols.clone()
    }
    /// Return a mutable reference to the symbols node which is currently processed
    pub fn current_node_mut(&mut self) -> RcMut<SymbolNode> {
        self.symbols.clone()
    }

    /// Add symbol to current symbol table
    pub fn add_symbol(&mut self, symbol: RcMut<SymbolNode>) {
        SymbolNode::insert_child(&mut self.symbols, symbol);
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
        let current_node = self.current_node();
        if let Some(child) =
            SymbolNode::search_up(&current_node.borrow(), &qualified_name.clone().into())
        {
            Ok(child)
        } else {
            Err(super::EvalError::SymbolNotFound(qualified_name.clone()))
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

    /// Find a symbol in the symbol table and add it at the currently processed node
    pub fn use_symbol(&mut self, qualified_name: &QualifiedName) -> EvalResult<()> {
        let current_node = self.current_node_mut();
        if let Some(child) =
            SymbolNode::search_up(&current_node.borrow(), &qualified_name.clone().into())
        {
            SymbolNode::insert_child(&mut self.current_node_mut(), child);
            Ok(())
        } else {
            Err(super::EvalError::SymbolNotFound(qualified_name.clone()))
        }
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
}

impl PushDiag for EvalContext {
    fn push_diag(&mut self, diag: Diag) -> EvalResult<()> {
        self.diag_handler.push_diag(diag)
    }
}

impl GetSourceFileByHash for EvalContext {
    fn get_source_file_by_hash(&self, hash: u64) -> Option<&SourceFile> {
        self.source_files.get_source_file_by_hash(hash)
    }
}
