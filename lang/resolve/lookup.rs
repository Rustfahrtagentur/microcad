// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{resolve::*, syntax::*};

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
        log::trace!(
            "{lookup} for symbol '{name:?}' within '{within}'",
            within = within.full_name(),
            lookup = crate::mark!(LOOKUP)
        );
        match (self.lookup(name), within.search(name, true)) {
            // found both
            (Ok(global), Ok(relative)) => {
                // check if one is an alias of the other
                match (global.is_alias(), relative.is_alias()) {
                    (true, false) => Ok(relative),
                    (false, true) => Ok(global),
                    (true, true) => unreachable!("found two aliases"),
                    (false, false) => {
                        if relative == global {
                            Ok(global)
                        } else {
                            Err(Self::ambiguity_error(
                                relative.full_name(),
                                [global.full_name()].into_iter().collect(),
                            ))
                        }
                    }
                }
            }
            // found one
            (Ok(symbol), Err(_)) | (Err(_), Ok(symbol)) => {
                log::trace!(
                    "{found} symbol '{name:?}' within '{within}'",
                    within = within.full_name(),
                    found = crate::mark!(FOUND_INTERIM)
                );
                Ok(symbol)
            }
            // found nothing
            (Err(err), Err(_)) => {
                log::trace!(
                    "{not_found} symbol '{name:?}' within '{within}'",
                    within = within.full_name(),
                    not_found = crate::mark!(NOT_FOUND_INTERIM)
                );
                Err(err)
            }
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

    /// Returns an error if name starts with `super::`.
    fn deny_super(&self, name: &QualifiedName) -> ResolveResult<()> {
        if name.count_super() > 0 {
            log::trace!(
                "{not_found} '{name:?}' is not canonical",
                not_found = crate::mark!(NOT_FOUND_INTERIM),
            );
            return Err(ResolveError::SymbolNotFound(name.clone()));
        }
        Ok(())
    }
}
