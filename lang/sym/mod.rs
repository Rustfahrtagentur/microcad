// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Symbol table

mod context;
mod stack;
mod sym_error;
mod symbol;
mod symbol_table;

pub use context::*;
pub use stack::*;
pub use sym_error::*;
pub use symbol::*;
pub use symbol_table::*;

/// Id type (base of all identifiers)
pub type Id = compact_str::CompactString;

/// Trait for symbols to identify themselves
pub trait Sym {
    /// Return id of this symbol
    fn id(&self) -> Option<Id>;
}

/// Trait of an element which contains other symbols
pub trait Symbols {
    /// fetch symbol with id
    fn fetch(&self, id: &Id) -> Option<std::rc::Rc<Symbol>>;

    /// Add a symbol
    fn add(&mut self, symbol: Symbol) -> &mut Self;

    /// Add an alias for a symbol
    fn add_alias(&mut self, symbol: Symbol, alias: Id) -> &mut Self;

    /// Copy symbols into another Instance
    fn copy<T: Symbols>(&self, into: &mut T) -> SymResult<()>;

    /// Shortcut to add a value symbol which can't be done via `.into()`
    #[cfg(test)]
    fn add_value(&mut self, id: Id, value: crate::eval::Value) -> &mut Self {
        self.add(Symbol::Value(id, value));
        self
    }
}
