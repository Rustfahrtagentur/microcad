// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Call stack for evaluation

use crate::{diag::*, eval::*, sym::*};

/// Context interface
pub trait Context {
    /// Push a new symbol table to the stack (enter a new scope)
    fn push(&mut self, stack_frame: StackFrame);

    /// Pop the top symbol table from the stack (exit the current scope)
    fn pop(&mut self);

    /// The top symbol table in the stack
    fn top(&self) -> SymResult<&StackFrame>;

    /// The top symbol table in the stack (mutable)
    fn top_mut(&mut self) -> &mut StackFrame;

    /// Create a new symbol table and push it to the stack
    fn scope<T>(
        &mut self,
        stack_frame: StackFrame,
        f: impl FnOnce(&mut Self) -> EvalResult<T>,
    ) -> EvalResult<T>;

    /// Read-only access to diagnostic handler
    fn diag(&self) -> &DiagHandler;
}
