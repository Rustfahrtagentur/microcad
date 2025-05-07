// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, resolve::*, src_ref::SrcReferrer, syntax::*};
use std::collections::btree_map::BTreeMap;

/// Map Id to SymbolNode reference
#[derive(Debug, Default, Clone)]
pub struct SymbolMap(BTreeMap<Identifier, Symbol>);

impl std::ops::Deref for SymbolMap {
    type Target = BTreeMap<Identifier, Symbol>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SymbolMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<&ArgumentMap> for SymbolMap {
    fn from(arg_map: &ArgumentMap) -> Self {
        let mut symbol_map = SymbolMap::default();
        for (id, value) in arg_map.iter() {
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
}

impl std::fmt::Display for SymbolMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (id, symbol) in self.0.iter() {
            symbol.print_symbol(f, Some(id), 0)?;
        }

        Ok(())
    }
}
