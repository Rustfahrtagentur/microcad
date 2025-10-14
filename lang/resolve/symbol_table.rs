// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use derive_more::{Deref, DerefMut};

use crate::{resolve::*, syntax::*};

/// *Symbol table* holding global and local symbols.
#[derive(Default, Deref, DerefMut)]
pub struct SymbolTable {
    /// Global symbols (including root).
    symbol_map: SymbolMap,
}

impl SymbolTable {
    /// Add a new symbol to the table
    pub fn add_symbol(&mut self, symbol: Symbol) -> ResolveResult<()> {
        self.insert_symbol(symbol.id(), symbol.clone())
    }

    /// Add another symbol map to the table
    pub fn add_symbol_map(&mut self, symbol_map: &SymbolMap) -> ResolveResult<()> {
        symbol_map
            .iter()
            .map(|(id, symbol)| (id.clone(), symbol.clone()))
            .try_for_each(|(id, symbol)| {
                assert!(!symbol.is_link());
                self.insert_symbol(id, symbol)
            })
    }

    /// Add a new symbol to the table
    pub fn insert_symbol(&mut self, id: Identifier, symbol: Symbol) -> ResolveResult<()> {
        log::trace!("insert symbol: {id}");
        if let Some(symbol) = self.symbol_map.insert(id, symbol.clone()) {
            Err(ResolveError::AmbiguousSymbol(symbol.id()))
        } else {
            Ok(())
        }
    }

    pub(super) fn values(&self) -> Symbols {
        self.symbol_map.values().cloned().collect()
    }

    /// Return a list of unchecked symbols
    ///
    /// Symbols only get *checked* if [check()] was called before!
    pub fn unchecked(&self) -> Symbols {
        let mut unchecked = Symbols::default();
        self.symbol_map
            .values()
            .for_each(|symbol| symbol.unchecked(&mut unchecked));
        unchecked
    }

    /// Return a list of unused symbols
    pub fn unused(&self) -> Symbols {
        let mut unchecked = Symbols::default();
        self.symbol_map
            .values()
            .for_each(|symbol| symbol.unused(&mut unchecked));
        unchecked
    }

    /// search all ids which require target mode (e.g. `assert_valid`)
    pub(super) fn search_target_mode_ids(&self) -> ResolveResult<IdentifierSet> {
        let mut ids = IdentifierSet::default();
        self.symbol_map
            .values()
            .try_for_each(|symbol| symbol.search_target_mode_ids(&mut ids))?;
        Ok(ids)
    }

    // Search recursively within symbol **and** in the symbol table (global)
    pub(super) fn lookup_within(
        &self,
        name: &QualifiedName,
        within: &Option<Symbol>,
    ) -> ResolveResult<Symbol> {
        if let Some(within) = within {
            match (within.search(name), self.lookup(name)) {
                (Ok(relative), Ok(global)) => {
                    if relative == global || relative.is_alias() {
                        Ok(global)
                    } else if global.is_alias() {
                        Ok(relative)
                    } else {
                        todo!("lookup ambiguous:\n  {relative:?}\n  {global:?}")
                    }
                }
                (Ok(symbol), Err(_)) | (Err(_), Ok(symbol)) => Ok(symbol),
                (Err(err), Err(_)) => Err(err),
            }
        } else {
            self.lookup(name)
        }
    }
}

impl WriteToFile for SymbolTable {}

impl Lookup for SymbolTable {
    /// Lookup a symbol from global symbols.
    fn lookup(&self, name: &QualifiedName) -> ResolveResult<Symbol> {
        log::trace!(
            "{lookup} for global symbol '{name:?}'",
            lookup = crate::mark!(LOOKUP)
        );
        let symbol = match self.symbol_map.search(name) {
            Ok(symbol) => symbol,
            Err(err) => {
                log::trace!(
                    "{not_found} global symbol: {name:?}",
                    not_found = crate::mark!(NOT_FOUND_INTERIM),
                );
                return Err(err)?;
            }
        };
        symbol.set_check();
        log::trace!(
            "{found} global symbol: {symbol:?}",
            found = crate::mark!(FOUND_INTERIM),
        );
        Ok(symbol)
    }
}

impl std::fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            self.symbol_map
                .iter()
                .map(|(_, symbol)| symbol)
                .filter(|symbol| !symbol.is_deleted())
                .map(|symbol| symbol.full_name().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl std::fmt::Debug for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.symbol_map)
    }
}
