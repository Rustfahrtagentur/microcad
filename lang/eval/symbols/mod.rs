// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod stack_frame;
mod stack;
mod source_cache;
mod symbol_table;

pub use stack_frame::*;
pub use stack::*;
pub use source_cache::*;
pub use symbol_table::*;

use crate::{eval::*, syntax::*};

/// Trait to handle symbol table.
pub trait Lookup {
    /// Lookup for local or global symbol by qualified name.
    ///
    /// - looks in local stack
    /// - looks in symbol map
    /// - follows aliases (use statements)
    /// - detect any ambiguity
    /// - loads external files
    fn lookup(&mut self, name: &QualifiedName) -> EvalResult<Symbol>;
}

/// Trait to manage the *local stack*.
///
/// The *local stack* holds a state about where evaluation is currently processing the source code.
///
/// Items on the local stack can be of different types:
/// - a *source file* with an own local stack frame,
/// - a *scope* (surrounded by curly brackets `{}`) within the source file,
/// - a *namespace* without local variables, or
/// - a *module* without local variables.
///
/// Each one may have different items it stores (see [`LocalFrame`]).
pub trait Locals {
    /// Open a new source scope with a new [local stack frame](LocalFrame).
    fn open_source(&mut self, id: Identifier);

    /// Open a new [object body](Body).
    fn open_body(&mut self);

    /// Open call.
    fn open_call(&mut self, symbol: Symbol, args: CallArgumentList, src_ref: impl SrcReferrer);

    /// Open a namespace.
    fn open_namespace(&mut self, id: Identifier);

    /// Close scope (stack pop).
    fn close(&mut self);

    /// Fetch a local variable from current stack frame.
    fn fetch(&self, id: &Identifier) -> EvalResult<Symbol>;

    /// Add a named local value to current locals.
    ///
    /// TODO: Is this special function really needed?
    fn add_local_value(&mut self, id: Identifier, value: Value) -> EvalResult<()>;
}
