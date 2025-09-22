// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{diag::*, resolve::*, syntax::*, value::*};
use derive_more::{Deref, DerefMut};
use std::collections::btree_map::BTreeMap;

/// Map Id to SymbolNode reference
#[derive(Debug, Default, Clone, Deref, DerefMut)]
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

impl FromIterator<(Identifier, Value)> for SymbolMap {
    fn from_iter<T: IntoIterator<Item = (Identifier, Value)>>(iter: T) -> Self {
        let mut symbol_map = SymbolMap::default();
        for (id, value) in iter {
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
                symbol.set_use();
                return Ok(symbol.clone());
            }
        } else {
            let (id, leftover) = name.split_first();
            if let Some(symbol) = self.get(&id) {
                symbol.set_use();
                if leftover.is_empty() {
                    log::trace!("Fetched {name:?} from globals (symbol map)");
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
            child.1.detach();
        }
        self
    }

    /// Print contained symbols with indention.
    pub fn print(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
        for (id, symbol) in self.0.iter() {
            symbol.print_symbol(f, Some(id), depth, true)?;
        }

        Ok(())
    }

    /// Collect all symbols engaged in that name.
    ///
    /// Example: `what`=`a::b::c` will return the symbols: `a`,`a::b` and `a::b::c`
    pub fn path_to(&self, what: &QualifiedName) -> ResolveResult<Symbols> {
        (1..(what.len() + 1))
            .map(|n| what[0..n].iter().cloned().collect())
            .map(|what| self.search(&what))
            .collect()
    }
}

impl std::fmt::Display for SymbolMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (id, symbol) in self.0.iter() {
            symbol.print_symbol(f, Some(id), 0, true)?;
        }

        Ok(())
    }
}

#[test]
fn symbol_map_path_to() {
    let mut symbols = SymbolMap::new();
    let a = Symbol::new(SymbolDefinition::Tester("a".into()), None);
    let b = Symbol::new(SymbolDefinition::Tester("b".into()), Some(a.clone()));
    let c = Symbol::new(SymbolDefinition::Tester("c".into()), Some(b.clone()));
    Symbol::add_child(&b, c);
    Symbol::add_child(&a, b);
    symbols.add_node(a);

    let name: QualifiedName = "a::b::c".into();

    log::trace!("symbols:\n{symbols}");
    let symbols = symbols.path_to(&name).expect("test error");
    log::trace!("parents of {name}: {}", symbols.full_names());
    assert_eq!(
        symbols.full_names().to_string(),
        "a, a::b, a::b::c".to_string(),
    );
}
