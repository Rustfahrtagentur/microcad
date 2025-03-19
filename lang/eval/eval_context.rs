// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{diag::*, parse::*, resolve::*, source_file_cache::*};

/// Context for evaluation
///
/// The context is used to store the current state of the evaluation.
/// A context is essentially a stack of symbol tables
pub struct EvalContext {
    /// Symbol table
    symbols: SymbolNodeRc,
    /// Current source file being evaluated
    current_source_file: Option<std::rc::Rc<SourceFile>>,
    /// Source file cache containing all source files loaded in the context
    source_files: SourceFileCache,
    /// Source file diagnostics
    diag_handler: DiagHandler,
}

impl EvalContext {
    /// Create a new context from a source file
    pub fn from_source_file(source_file: std::rc::Rc<SourceFile>) -> Self {
        let mut ctx = Self {
            current_source_file: Some(source_file.clone()),
            symbols: SymbolNode::new(SymbolDefinition::SourceFile(source_file.clone()), None),
            source_files: Default::default(),
            diag_handler: Default::default(),
        };

        ctx.source_files.add(source_file);
        ctx
    }

    /// Return the current source file
    ///
    /// Note: This should not be an optional value, as the context is always created with a source file
    pub fn current_source_file(&self) -> Option<std::rc::Rc<SourceFile>> {
        self.current_source_file.clone()
    }

    /// Add source file to Context
    pub fn add_source_file(&mut self, source_file: SourceFile) {
        self.source_files.add(std::rc::Rc::new(source_file))
    }

    /// Error with stack trace
    pub fn error_with_stack_trace(
        &mut self,
        src_ref: impl crate::src_ref::SrcReferrer,
        error: impl std::error::Error + 'static,
    ) -> crate::eval::EvalResult<()> {
        self.error(src_ref, Box::new(error))
    }
}

impl PushDiag for EvalContext {
    fn push_diag(&mut self, diag: Diag) -> crate::eval::EvalResult<()> {
        self.diag_handler.push_diag(diag)
    }
}

impl GetSourceFileByHash for EvalContext {
    fn get_source_file_by_hash(&self, hash: u64) -> Option<&SourceFile> {
        self.source_files.get_source_file_by_hash(hash)
    }
}
