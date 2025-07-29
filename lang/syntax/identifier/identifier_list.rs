// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{src_ref::*, syntax::*};
use derive_more::{Deref, DerefMut};

/// A list of identifiers
///
/// Used e.g. for multiple variable declarations.
/// Cannot contain duplicates.
#[derive(Default, Clone, PartialEq, Deref, DerefMut)]
pub struct IdentifierList(pub Refer<Vec<Identifier>>);

impl IdentifierList {
    /// Create new identifier list
    pub fn new(identifiers: Vec<Identifier>, src_ref: SrcRef) -> Self {
        Self(Refer::new(identifiers, src_ref))
    }
}

impl SrcReferrer for IdentifierList {
    fn src_ref(&self) -> identifier::SrcRef {
        self.0.src_ref()
    }
}

impl FromIterator<Identifier> for IdentifierList {
    fn from_iter<T: IntoIterator<Item = Identifier>>(iter: T) -> Self {
        let v: Vec<_> = iter.into_iter().collect();
        let src_ref = SrcRef::merge_all(v.iter());
        Self(Refer::new(v, src_ref))
    }
}

impl std::fmt::Display for IdentifierList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut sorted = self.0.clone();
        sorted.sort();
        write!(
            f,
            "{}",
            sorted
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl std::fmt::Debug for IdentifierList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut sorted = self.0.clone();
        sorted.sort();
        write!(
            f,
            "{}",
            sorted
                .iter()
                .map(|id| format!("{id:?}"))
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl std::iter::IntoIterator for IdentifierList {
    type Item = Identifier;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
