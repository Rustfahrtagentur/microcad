// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod stack;
mod stack_frame;

pub use stack::*;
pub use stack_frame::*;

use crate::{eval::*, model::*, syntax::*};

/// Trait to manage the *locals*.
///
/// The *locals* manage the state about where evaluation is currently processing the source code.
///
/// Items on the local stack can be of different types:
/// - a *source file* with an own local stack frame,
/// - a *body* (surrounded by curly brackets `{}`),
/// - a *module* without local variables but aliases (use statements), or
/// - a *call* without local variables.
///
/// Each one may have different items it stores (see [`StackFrame`]).
pub trait Locals {
    /// Don't use this function directly.
    fn open(&mut self, frame: StackFrame);

    /// Close scope (stack pop).
    fn close(&mut self);

    /// Fetch a local variable from current stack frame.
    fn fetch(&self, id: &Identifier) -> EvalResult<Symbol>;

    /// Set/add a named local value to current locals.
    fn set_local_value(&mut self, id: Identifier, value: Value) -> EvalResult<()>;

    /// Get a named local value from locals.
    fn get_local_value(&self, id: &Identifier) -> EvalResult<Value>;

    /// Add a symbol to current locals.
    fn add_symbol(&mut self, id: Identifier, symbol: Symbol) -> EvalResult<()>;

    /// Get a property value from current model.
    fn get_model(&self) -> EvalResult<Model>;

    /// Return qualified name of current module or workbench.
    fn current_name(&self) -> QualifiedName;
}
