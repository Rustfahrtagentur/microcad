// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod symbol;
mod symbol_table;

pub use symbol::*;
pub use symbol_table::*;

use crate::eval::*;

/// Trait for symbols to identify themselves
pub trait Sym {
    /// Return id of this symbol
    fn id(&self) -> Option<microcad_core::Id>;
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
    fn copy<T: Symbols>(&self, into: &mut T);

    /// Shortcut to add a value symbol which can't be done via `.into()`
    #[cfg(test)]
    fn add_value(&mut self, id: Id, value: Value) -> &mut Self {
        self.add(Symbol::Value(id, value));
        self
    }
}
