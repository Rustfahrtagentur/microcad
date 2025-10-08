// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use derive_more::{Deref, DerefMut};

use crate::{resolve::*, syntax::*};

/// *Symbol table* holding global and local symbols.
///
/// The symbol table consists of the following members:
///
/// - One *root [`Symbol`]* resolved from the *initial source file*.
/// - A [`SourceCache`] of all *loaded source files* (accessible by *qualified name*, *file path* and *source hash*).
/// - A [`Stack`] of [`StackFrame`]s.
/// - A [`SymbolMap`] of all *global symbols*.
///
/// All these internal structures can be accessed by several implemented traits.
#[derive(Default, Deref, DerefMut)]
pub struct SymbolTable {
    /// Global symbols (including root).
    symbols: SymbolMap,
}

impl SymbolTable {
    /// Collect all symbols engaged in that name.
    ///
    /// Example: `what`=`a::b::c` will return the symbols: `a`,`a::b` and `a::b::c`
    pub fn path_to(&self, what: &QualifiedName) -> ResolveResult<Symbols> {
        self.symbols.path_to(what)
    }

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
        log::trace!("insert symbol: {}", id);
        if let Some(symbol) = self.symbols.insert(id, symbol.clone()) {
            Err(ResolveError::AmbiguousSymbol(symbol.id()))
        } else {
            Ok(())
        }
    }

    pub(super) fn values(&self) -> Symbols {
        self.symbols.values().cloned().collect()
    }

    /// Return a list of unchecked symbols
    ///
    /// Symbols only get *checked* if [check()] was called before!
    pub fn unchecked(&self) -> Symbols {
        let mut unchecked = Symbols::default();
        self.symbols
            .values()
            .for_each(|symbol| symbol.unchecked(&mut unchecked));
        unchecked
    }

    /// Return a list of unused symbols
    pub fn unused(&self) -> Symbols {
        let mut unchecked = Symbols::default();
        self.symbols
            .values()
            .for_each(|symbol| symbol.unused(&mut unchecked));
        unchecked
    }
}

impl WriteToFile for SymbolTable {}

impl Lookup for SymbolTable {
    /// Lookup a symbol from global symbols.
    fn lookup(&self, name: &QualifiedName) -> ResolveResult<Symbol> {
        log::trace!("Looking for global symbol '{name:?}'");
        let symbol = match self.symbols.search(name) {
            Ok(symbol) => symbol,
            Err(err) => {
                log::trace!(
                    "{not_found} global symbol: {name:?}",
                    not_found = crate::mark!(NOT_FOUND_INTERIM),
                );
                return Err(err)?;
            }
        };
        log::trace!(
            "{found} global symbol: {symbol:?}",
            found = crate::mark!(FOUND),
        );
        Ok(symbol)
    }
}

impl std::fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            self.symbols
                .iter()
                .map(|symbol| symbol.1.full_name().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl std::fmt::Debug for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.symbols)
    }
}
