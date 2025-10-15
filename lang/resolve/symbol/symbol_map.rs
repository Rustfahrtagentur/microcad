// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{diag::*, resolve::*, syntax::*, value::*};
use derive_more::{Deref, DerefMut};
use indexmap::IndexMap;

/// Map Id to SymbolNode reference
#[derive(Default, Clone, Deref, DerefMut)]
pub struct SymbolMap(IndexMap<Identifier, Symbol>);

impl From<Tuple> for SymbolMap {
    fn from(tuple: Tuple) -> Self {
        Self::from_iter(
            tuple
                .named
                .iter()
                .map(|(id, value)| (id.clone(), value.clone())),
        )
    }
}

impl FromIterator<(Identifier, Value)> for SymbolMap {
    fn from_iter<T: IntoIterator<Item = (Identifier, Value)>>(iter: T) -> Self {
        iter.into_iter()
            .map(|(id, value)| {
                (
                    id.clone(),
                    Symbol::new(SymbolDefinition::Argument(id.clone(), value.clone()), None),
                )
            })
            .collect()
    }
}

impl FromIterator<(Identifier, Symbol)> for SymbolMap {
    fn from_iter<T: IntoIterator<Item = (Identifier, Symbol)>>(iter: T) -> Self {
        SymbolMap(iter.into_iter().collect())
    }
}

impl WriteToFile for SymbolMap {}

impl SymbolMap {
    /// Insert a not by it's own id.
    pub fn add_node(&mut self, symbol: Symbol) {
        self.0.insert(symbol.id(), symbol);
    }

    pub fn get<'a>(&'a self, id: &Identifier) -> Option<&'a Symbol> {
        self.iter()
            .filter(|(_, symbol)| !symbol.is_deleted())
            .find(|(i, _)| *i == id)
            .map(|(_, symbol)| symbol)
    }

    /// Search for a symbol in symbol map.
    pub(crate) fn search(&self, name: &QualifiedName, respect: bool) -> ResolveResult<Symbol> {
        log::trace!("Searching {name:?} in symbol map");
        let (id, leftover) = name.split_first();
        if let Some(symbol) = self.get(&id) {
            if leftover.is_empty() {
                log::trace!("Fetched {name:?} from symbol map");
                Ok(symbol.clone())
            } else {
                symbol.search(&leftover, respect)
            }
        } else {
            Err(ResolveError::SymbolNotFound(name.clone()))
        }
    }

    pub(super) fn resolve_all(&self, context: &mut ResolveContext) -> ResolveResult<SymbolMap> {
        let mut from_children = SymbolMap::default();
        self.values()
            .filter(|child| child.is_resolvable())
            .flat_map(|child| child.resolve(context))
            .for_each(|map| from_children.extend(map.iter().map(|(k, v)| (k.clone(), v.clone()))));
        Ok(from_children)
    }
}

impl std::fmt::Display for SymbolMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .try_for_each(|(id, symbol)| symbol.print_symbol(f, Some(id), 0, false, true))
    }
}

impl std::fmt::Debug for SymbolMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .try_for_each(|(id, symbol)| symbol.print_symbol(f, Some(id), 0, true, true))
    }
}
