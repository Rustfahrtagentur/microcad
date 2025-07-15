// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, resolve::*, src_ref::SrcReferrer, syntax::*, value::*};
use derive_more::{Deref, DerefMut};
use std::collections::btree_map::BTreeMap;

/// Map Id to SymbolNode reference
#[derive(Debug, Default, Clone, Deref, DerefMut)]
pub struct SymbolMap(BTreeMap<Identifier, Symbol>);

impl From<&Tuple> for SymbolMap {
    fn from(tuple: &Tuple) -> Self {
        let mut symbol_map = SymbolMap::default();
        for (id, value) in tuple.named.iter() {
            symbol_map.add_node(Symbol::new_call_argument(id.clone(), value.clone()))
        }
        symbol_map
    }
}

impl SymbolMap {
    /// Create symbol new map
    pub fn new() -> Self {
        Self(Default::default())
    }

    /// Insert a not by it's own id.
    pub fn add_node(&mut self, symbol: Symbol) {
        let id = symbol.id();
        self.0.insert(id, symbol);
    }

    /// Insert a not by it's own id.
    pub fn insert_node(&mut self, id: Identifier, symbol: Symbol) {
        self.0.insert(id, symbol);
    }

    /// Search for a symbol in symbol map.
    pub fn search(&self, name: &QualifiedName) -> EvalResult<Symbol> {
        if name.is_empty() {
            return Err(EvalError::NotAName(name.src_ref()));
        }

        let (id, leftover) = name.split_first();
        if let Some(symbol) = self.get(&id) {
            if leftover.is_empty() {
                log::trace!("Fetched {name} from globals (symbol map)");
                return Ok(symbol.clone());
            } else if let Some(symbol) = symbol.search(&leftover) {
                return Ok(symbol);
            }
        }

        Err(EvalError::SymbolNotFound(name.clone()))
    }

    /// detach children from their parent
    pub fn detach_from_parent(mut self) -> Self {
        for child in self.iter_mut() {
            child.1.borrow_mut().parent = None;
        }
        self
    }

    /// Print contained symbols with indention.
    pub fn print(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
        for (id, symbol) in self.0.iter() {
            symbol.print_symbol(f, Some(id), depth)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for SymbolMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (id, symbol) in self.0.iter() {
            symbol.print_symbol(f, Some(id), 0)?;
        }

        Ok(())
    }
}
