// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod local_frame;
mod local_stack;
mod source_cache;
mod symbol_table;

pub use local_frame::*;
pub use local_stack::*;
pub use source_cache::*;
pub use symbol_table::*;

use crate::{eval::*, syntax::*};

/// Trait to handle symbol table
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

/// Trait to manage local stack
pub trait Locals {
    /// Open a new source (stack push).
    fn open_source(&mut self, id: Identifier);

    /// Open a new scope (stack push).
    fn open_scope(&mut self);

    /// Open a new scope (stack push).
    fn open_namespace(&mut self, id: Identifier);

    /// Open a new scope (stack push).
    fn open_module(&mut self, id: Identifier);

    /// Close scope (stack pop).
    fn close(&mut self);

    /// Fetch a local variable from current stack frame.
    fn fetch(&self, id: &Identifier) -> EvalResult<Symbol>;

    /// Add a named local value to current locals.
    ///
    /// TODO: Is this special function really needed?
    fn add_local_value(&mut self, id: Identifier, value: Value) -> EvalResult<()>;
}
