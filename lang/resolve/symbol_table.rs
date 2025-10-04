// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

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
#[derive(Default)]
pub struct SymbolTable {
    /// Global symbols (including root).
    symbols: SymbolMap,
}

impl SymbolTable {
    /// Solve any alias within the given qualified name.
    ///
    /// # Example
    /// ```µcad
    /// mod my {
    ///   use std::geo2d;
    /// }
    /// my::geo2d::rect(1mm);
    /// ```
    pub fn de_alias(&self, name: &QualifiedName) -> QualifiedName {
        (1..name.len())
            .rev()
            .filter_map(|p| {
                if let Ok(symbol) = self.lookup(&QualifiedName::no_ref(name[0..p].to_vec())) {
                    Some((p, symbol))
                } else {
                    None
                }
            })
            .find_map(|(p, symbol)| {
                symbol.with_def(|def| {
                    if let SymbolDefinition::Alias(.., alias) = def {
                        let suffix: QualifiedName = name[p..].iter().cloned().collect();
                        let new_name = suffix.with_prefix(alias);
                        log::trace!("De-aliased name: {name:?} into {new_name:?}");
                        Some(new_name)
                    } else {
                        None
                    }
                })
            })
            .unwrap_or(name.clone())
    }

    /// If given symbol is an alias returns the target or the symbol itself if not.
    pub fn follow_alias(&self, symbol: &Symbol) -> ResolveResult<Symbol> {
        // execute alias from any use statement
        symbol.with_def(|def| {
            if let SymbolDefinition::Alias(.., name) = def {
                log::trace!("{found} alias => {name:?}", found = crate::mark!(FOUND));
                Ok(self.lookup(name)?)
            } else {
                Ok(symbol.clone())
            }
        })
    }

    /// Collect all symbols engaged in that name.
    ///
    /// Example: `what`=`a::b::c` will return the symbols: `a`,`a::b` and `a::b::c`
    pub fn path_to(&self, what: &QualifiedName) -> ResolveResult<Symbols> {
        self.symbols.path_to(what)
    }

    /// Add a new symbol to the table
    pub fn add_symbol(&mut self, symbol: Symbol) -> ResolveResult<()> {
        log::trace!("add symbol: {}", symbol.id());
        if let Some(symbol) = self.symbols.insert(symbol.id(), symbol.clone()) {
            Err(ResolveError::AmbiguousSymbol(symbol.id()))
        } else {
            Ok(())
        }
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
        let name = self.de_alias(name);
        let symbol = match self.symbols.search(&name) {
            Ok(symbol) => symbol.clone(),
            Err(err) => return Err(err)?,
        };
        log::trace!(
            "{found} global symbol: {symbol}",
            found = crate::mark!(FOUND),
        );
        Ok(symbol)
    }
}

impl std::fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.symbols)
    }
}

impl std::fmt::Debug for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.symbols)
    }
}
