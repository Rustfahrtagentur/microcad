// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod source_cache;
mod stack;
mod stack_frame;
mod symbol_table;

pub use source_cache::*;
pub use stack::*;
pub use stack_frame::*;
pub use symbol_table::*;

use crate::{eval::*, model::*, rc::*, syntax::*};

/// Trait to handle symbol table.
pub trait Lookup {
    /// Lookup for local or global symbol by qualified name.
    ///
    /// - looks on *stack*
    /// - looks in *symbol table*
    /// - follows *aliases* (use statements)
    /// - detect any ambiguity
    /// - loads *external files*
    fn lookup(&mut self, name: &QualifiedName) -> EvalResult<Symbol>;
}

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

    /// Get current model.
    fn get_model_builder(&self) -> EvalResult<RcMut<ModelBuilder>>;
}
