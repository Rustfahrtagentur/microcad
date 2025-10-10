// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{diag::*, resolve::*, syntax::*, value::*};
use derive_more::{Deref, DerefMut};
use indexmap::IndexMap;

pub(super) enum Link {
    None,
    Alias(QualifiedName),
}

/// Map Id to SymbolNode reference
#[derive(Default, Clone, Deref, DerefMut)]
pub struct SymbolMap(IndexMap<Identifier, Symbol>);

impl From<Tuple> for SymbolMap {
    fn from(tuple: Tuple) -> Self {
        let mut symbol_map = SymbolMap::default();
        for (id, value) in tuple.named.iter() {
            symbol_map.add_node(Symbol::new(
                SymbolDefinition::Argument(id.clone(), value.clone()),
                None,
            ))
        }
        symbol_map
    }
}

impl FromIterator<(Identifier, Value)> for SymbolMap {
    fn from_iter<T: IntoIterator<Item = (Identifier, Value)>>(iter: T) -> Self {
        let mut symbol_map = SymbolMap::default();
        for (id, value) in iter {
            symbol_map.add_node(Symbol::new(
                SymbolDefinition::Argument(id.clone(), value.clone()),
                None,
            ))
        }
        symbol_map
    }
}

impl FromIterator<(Identifier, Symbol)> for SymbolMap {
    fn from_iter<T: IntoIterator<Item = (Identifier, Symbol)>>(iter: T) -> Self {
        let mut symbol_map = SymbolMap::default();
        for (id, symbol) in iter {
            symbol_map.insert(id, symbol);
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

    pub fn get_not_deleted<'a>(&'a self, id: &Identifier) -> Option<&'a Symbol> {
        self.iter()
            .filter(|(_, symbol)| !symbol.is_deleted())
            .find(|(i, _)| *i == id)
            .map(|(_, symbol)| symbol)
    }

    /// Search for a symbol in symbol map.
    pub(crate) fn search(&self, name: &QualifiedName) -> ResolveResult<Symbol> {
        log::trace!("Searching {name:?} in symbol map");
        if name.is_empty() {
            if let Some(symbol) = self.get_not_deleted(&Identifier::none()) {
                log::trace!("Fetched {name:?} from globals (symbol map)");
                return Ok(symbol.clone());
            }
        } else {
            let (id, leftover) = name.split_first();
            if let Some(symbol) = self.get_not_deleted(&id) {
                match symbol.get_link() {
                    Link::None => (),
                    Link::Alias(alias) => {
                        if let Some(parent) = symbol.get_parent() {
                            let symbol = {
                                let relative = parent.search(&alias);
                                let absolute = self.search(&alias);
                                match (absolute, relative) {
                                    (Ok(absolute), Ok(relative)) => {
                                        if absolute.is_deleted() {
                                            Ok(relative)
                                        } else if relative.is_deleted() {
                                            Ok(absolute)
                                        } else {
                                            todo!("ambiguous")
                                        }
                                    }
                                    (Ok(symbol), Err(_)) | (Err(_), Ok(symbol)) => Ok(symbol),
                                    (Err(err), Err(_)) => Err(err),
                                }
                            }?;
                            return symbol.search(&leftover);
                        }
                    }
                }

                if leftover.is_empty() {
                    log::trace!("Fetched {name:?} from globals (symbol map)");
                    return Ok(symbol.clone());
                } else {
                    return symbol.search(&leftover);
                }
            }
        }

        Err(ResolveError::SymbolNotFound(name.clone()))
    }

    fn merge_all<I>(iter: I) -> SymbolMap
    where
        I: IntoIterator<Item = SymbolMap>,
    {
        let mut merged = SymbolMap::new();
        for map in iter {
            merged.extend(map.iter().map(|(k, v)| (k.clone(), v.clone())));
        }
        merged
    }

    pub(super) fn resolve_all(&self, context: &mut ResolveContext) -> ResolveResult<SymbolMap> {
        let from_children: SymbolMap = Self::merge_all(
            self.values()
                .filter(|child| child.is_resolvable())
                .flat_map(|child| child.resolve(context)),
        );
        Ok(from_children)
    }
}

impl std::fmt::Display for SymbolMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (id, symbol) in self.0.iter() {
            symbol.print_symbol(f, Some(id), 0, false, true)?;
        }

        Ok(())
    }
}

impl std::fmt::Debug for SymbolMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (id, symbol) in self.0.iter() {
            symbol.print_symbol(f, Some(id), 0, true, true)?;
        }

        Ok(())
    }
}
