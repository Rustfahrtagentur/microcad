// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{diag::*, resolve::*, syntax::*, value::*};
use derive_more::{Deref, DerefMut};
use std::collections::btree_map::BTreeMap;

/// Map Id to SymbolNode reference
#[derive(Debug, Default, Clone, Deref, DerefMut, serde::Serialize, serde::Deserialize)]
pub struct SymbolMap(BTreeMap<Identifier, Symbol>);

impl From<Tuple> for SymbolMap {
    fn from(tuple: Tuple) -> Self {
        let mut symbol_map = SymbolMap::default();
        for (id, value) in tuple.named.iter() {
            symbol_map.add_node(Symbol::new_call_argument(id.clone(), value.clone()))
        }
        symbol_map
    }
}

impl WriteToFile for SymbolMap {}

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
    pub fn search(&self, name: &QualifiedName) -> ResolveResult<Symbol> {
        if name.is_empty() {
            if let Some(symbol) = self.get(&Identifier::none()) {
                return Ok(symbol.clone());
            }
        } else {
            let (id, leftover) = name.split_first();
            if let Some(symbol) = self.get(&id) {
                if leftover.is_empty() {
                    log::trace!("Fetched {name} from globals (symbol map)");
                    return Ok(symbol.clone());
                } else if let Some(symbol) = symbol.search(&leftover) {
                    return Ok(symbol);
                }
            }
        }

        Err(ResolveError::SymbolNotFound(name.clone()))
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

    /// Move all children from another symbol into this map.
    /// # Arguments
    /// - `from`: Append this symbol's children
    ///
    /// Technically, nothing will be moved here because of the `Rc<RefCell<>>`,
    /// but by resetting the parent of all moved  children, those will see
    /// themselves as root symbols.
    pub fn move_children(&mut self, from: &Symbol) {
        // copy children
        from.borrow().children.iter().for_each(|(id, child)| {
            child.borrow_mut().parent = None;
            self.insert(id.clone(), child.clone());
        });
    }
}

impl std::fmt::Display for SymbolMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (id, symbol) in self.0.iter() {
            symbol.print_symbol(f, Some(id), 4)?;
        }

        Ok(())
    }
}
