// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use crate::{diag::*, parse::*, resolve::*, source_file_cache::*};

/// Context for evaluation
///
/// The context is used to store the current state of the evaluation.
/// A context is essentially a stack of symbol tables
pub struct EvalContext {
    /// Tree of all evaluated symbols
    symbols: RcMut<SymbolNode>,
    /// Stack of currently opened scopes with local symbols while evaluation
    scope_stack: ScopeStack,
    /// Symbol node which is currently evaluated
    current_symbol_node: Option<RcMut<SymbolNode>>,
    /// Current source file being evaluated
    current_source_file: Option<Rc<SourceFile>>,
    /// Source file cache containing all source files loaded in the context
    source_files: SourceFileCache,
    /// Source file diagnostics
    diag_handler: DiagHandler,
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
            current_symbol_node: Default::default(),
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

    /// Error with stack trace
    pub fn error_with_stack_trace(
        &mut self,
        src_ref: impl crate::src_ref::SrcReferrer,
        error: impl std::error::Error + 'static,
    ) -> EvalResult<()> {
        self.error(src_ref, Box::new(error))
    }

    /// Return a mutable reference to the symbols node which is currently processed
    pub fn current_node_mut(&mut self) -> RcMut<SymbolNode> {
        self.symbols.clone()
    }

    /// Find a symbol in the symbol table and add it at the currently processed node
    pub fn use_symbol(&mut self, qualified_name: &QualifiedName) -> EvalResult<()> {
        let current_node = self.current_node_mut();
        if let Some(child) =
            SymbolNode::search_up(&current_node.borrow(), &qualified_name.clone().into())
        {
            SymbolNode::insert_child(self.current_node_mut(), child);
            Ok(())
        } else {
            Err(super::EvalError::SymbolNotFound(qualified_name.clone()))
        }
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
