// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::resolve::*;
use custom_debug::Debug;
use derive_more::{Deref, DerefMut};

/// List of qualified names which can pe displayed
#[derive(Debug, Deref, DerefMut, Default)]
pub struct Symbols(Vec<Symbol>);

impl Symbols {
    /// Return all fully qualified names of all symbols.
    #[cfg(test)]
    pub(super) fn full_names(&self) -> QualifiedNames {
        self.iter().map(|symbol| symbol.full_name()).collect()
    }
}

impl FromIterator<Symbols> for Symbols {
    fn from_iter<T: IntoIterator<Item = Symbols>>(iter: T) -> Self {
        let mut symbols = Self::default();
        iter.into_iter()
            .for_each(|mut children| symbols.append(&mut children));
        symbols
    }
}

impl From<Vec<Symbol>> for Symbols {
    fn from(value: Vec<Symbol>) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Symbols {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|symbol| symbol.to_string())
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

impl FromIterator<Symbol> for Symbols {
    fn from_iter<T: IntoIterator<Item = Symbol>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
