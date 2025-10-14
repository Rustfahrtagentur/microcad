// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use derive_more::{Deref, DerefMut};

use crate::{resolve::*, syntax::*};

/// *Symbol table* holding global symbols.
#[derive(Default, Deref, DerefMut)]
pub struct SymbolTable(SymbolMap);

impl SymbolTable {
    /// Add a new symbol to the table
    pub fn add_symbol(&mut self, symbol: Symbol) -> ResolveResult<()> {
        self.insert_symbol(symbol.id(), symbol.clone())
    }

    /// Add a new symbol to the table
    pub fn insert_symbol(&mut self, id: Identifier, symbol: Symbol) -> ResolveResult<()> {
        log::trace!("insert symbol: {id}");
        if let Some(symbol) = self.insert(id, symbol.clone()) {
            Err(ResolveError::SymbolAlreadyDefined(symbol.full_name()))
        } else {
            Ok(())
        }
    }

    pub(super) fn symbols(&self) -> Symbols {
        self.values().cloned().collect()
    }

    /// Return a list of symbols which could not or have not been checked.
    pub fn unchecked(&self) -> Symbols {
        let mut unchecked = Symbols::default();
        self.values()
            .for_each(|symbol| symbol.unchecked(&mut unchecked));
        unchecked
    }

    /// Return a list of unused symbols
    ///
    /// Use this after eval for any useful result.
    pub fn unused(&self) -> Symbols {
        let mut unchecked = Symbols::default();
        self.values()
            .for_each(|symbol| symbol.unused(&mut unchecked));
        unchecked
    }

    /// Search all ids which require target mode (e.g. `assert_valid`)
    pub(super) fn search_target_mode_ids(&self) -> ResolveResult<IdentifierSet> {
        let mut ids = IdentifierSet::default();
        self.values()
            .try_for_each(|symbol| symbol.search_target_mode_ids(&mut ids))?;
        Ok(ids)
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
        let symbol = match self.search(name) {
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

    fn ambiguity_error(ambiguous: QualifiedName, others: QualifiedNames) -> ResolveError {
        ResolveError::AmbiguousSymbol(ambiguous, others)
    }
}

impl std::fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            self.iter()
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
        writeln!(f, "{:?}", self.0)
    }
}
