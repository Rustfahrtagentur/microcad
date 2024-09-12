// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod symbol;
mod symbol_table;

pub use symbol::*;
pub use symbol_table::*;

use crate::{eval::*, parse::*};

/// Trait for symbols to identify themselves
pub trait Sym {
    /// Return id of this symbol
    fn id(&self) -> Option<microcad_core::Id>;
}

pub trait Symbols {
    /// fetch all symbols which match id
    fn fetch_symbols(&self, id: &Id) -> Vec<&Symbol>;
    /// add symbol
    fn add_symbol(&mut self, symbol: Symbol) -> &mut Self;
    fn copy_symbols<T: Symbols>(&self, into: &mut T);

    fn add_value(&mut self, id: Id, value: Value) -> &mut Self {
        self.add_symbol(Symbol::Value(id, value));
        self
    }
    fn add_function(&mut self, f: std::rc::Rc<FunctionDefinition>) -> &mut Self {
        self.add_symbol(Symbol::Function(f));
        self
    }
    fn add_module(&mut self, m: std::rc::Rc<ModuleDefinition>) -> &mut Self {
        self.add_symbol(Symbol::ModuleDefinition(m));
        self
    }
    fn add_namespace(&mut self, n: std::rc::Rc<NamespaceDefinition>) -> &mut Self {
        self.add_symbol(Symbol::NamespaceDefinition(n));
        self
    }
    fn add_builtin_function(&mut self, f: BuiltinFunction) -> &mut Self {
        self.add_symbol(Symbol::BuiltinFunction(f));
        self
    }
    fn add_builtin_module(&mut self, m: BuiltinModule) -> &mut Self {
        self.add_symbol(Symbol::BuiltinModule(m));
        self
    }
}
