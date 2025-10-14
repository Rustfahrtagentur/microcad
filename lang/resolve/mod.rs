// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Single symbol resolving
//!
//! After parsing a source file (see [`mod@crate::parse`]) it must be resolved to get a symbol out of it:
//!
//! ```no_run
//! use microcad_lang::{syntax::*, parse::*, resolve::*};
//! let source_file = SourceFile::load("my.µcad").expect("parsing success");
//! let source_symbol = source_file.resolve();
//! ```
//!
//! To "run" the source file (and get the expected output) it must now be evaluated (see [`crate::eval`])  .

mod externals;
mod grant;
mod names;
mod resolve_context;
mod resolve_error;
mod sources;
mod symbol;
mod symbol_table;
mod symbolize;

use crate::{diag::*, syntax::*};
pub use externals::*;
pub use resolve_context::*;
pub use resolve_error::*;
pub use sources::*;
pub use symbol::*;
pub use symbol_table::*;

use grant::*;
use names::*;

/// Trait to lookup symbols by *qualified name*.
pub trait Lookup<E: std::error::Error = ResolveError> {
    /// Search a *symbol* by it's *qualified name*.
    /// # Arguments
    /// - `name`: *Qualified name* to search for.
    fn lookup(&self, name: &QualifiedName) -> Result<Symbol, E>;

    /// Return an ambiguity error.
    fn ambiguity_error(ambiguous: QualifiedName, others: QualifiedNames) -> E;

    /// Search a *symbol* by it's *qualified name* **and** within the given *symbol*.
    ///
    /// # Arguments
    /// - `name`: *Qualified name* to search for.
    /// - `within`: Searches within this *symbol* too.
    /// # Return
    /// If both are found and one is an *alias* returns the other one.
    fn lookup_within(&self, name: &QualifiedName, within: &Symbol) -> Result<Symbol, E> {
        match (self.lookup(name), within.search(name)) {
            // found both
            (Ok(global), Ok(relative)) => {
                if relative == global {
                    Ok(global)
                } else {
                    // check if one is an alias of the other
                    match (global.is_alias(), relative.is_alias()) {
                        (true, false) => Ok(relative),
                        (false, true) => Ok(global),
                        (true, true) => unreachable!("found two aliases"),
                        (false, false) => Err(Self::ambiguity_error(
                            relative.full_name(),
                            [global.full_name()].into_iter().collect(),
                        )),
                    }
                }
            }
            // found one
            (Ok(symbol), Err(_)) | (Err(_), Ok(symbol)) => Ok(symbol),
            // found nothing
            (Err(err), Err(_)) => Err(err),
        }
    }

    /// Search a *symbol* by it's *qualified name* **and** within a given *symbol*
    ///
    /// # Arguments
    /// - `name`: *qualified name* to search for
    /// - `within`: If some, searches within this *symbol* too.
    /// # Return
    /// If both are found and one is an *alias* returns the other one.
    fn lookup_within_opt(
        &self,
        name: &QualifiedName,
        within: &Option<Symbol>,
    ) -> Result<Symbol, E> {
        if let Some(within) = within {
            self.lookup_within(name, within)
        } else {
            self.lookup(name)
        }
    }

    /// Search a *symbol* by it's *qualified name* **and** within any of the given *symbol*s.
    ///
    /// # Arguments
    /// - `name`: *qualified name* to search for
    /// - `within`: If some, searches in this *symbol* too.
    /// # Return
    /// If multiple were found returns the one which is no aliases.
    fn lookup_within_many(&self, name: &QualifiedName, within: &Symbols) -> Result<Symbol, E> {
        assert!(!within.is_empty());

        let found: Vec<_> = within
            .iter()
            .map(|within| self.lookup_within(name, within))
            .chain([self.lookup(name)])
            .filter_map(|result| result.ok())
            .filter(|symbol| !symbol.is_alias())
            .collect();

        if found.len() > 1 {
            todo!("ambiguous");
        }

        if let Some(first) = found.first() {
            Ok(first.clone())
        } else {
            todo!("not found");
        }
    }

    /// Search a *symbol* by it's *qualified name* **and** within a *symbol* given by name.
    ///
    /// If both are found
    /// # Arguments
    /// - `name`: *qualified name* to search for.
    /// - `within`: Searches in the *symbol* with this name too.
    fn lookup_within_name(
        &self,
        name: &QualifiedName,
        within: &QualifiedName,
    ) -> Result<Symbol, E> {
        self.lookup_within(name, &self.lookup(within)?)
    }
}

/// Trait for items which can be fully qualified.
pub trait FullyQualify {
    /// Get a fully (up to root of symbol map) qualified name.
    fn full_name(&self) -> QualifiedName;
}
