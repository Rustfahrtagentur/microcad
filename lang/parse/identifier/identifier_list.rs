// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, src_ref::*};

/// A list of identifiers
///
/// Used e.g. for multiple variable declarations.
/// Cannot contain duplicates.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct IdentifierList(Refer<Vec<Identifier>>);

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

impl From<Vec<Identifier>> for IdentifierList {
    fn from(value: Vec<Identifier>) -> Self {
        Self(Refer::new(value, SrcRef(None)))
    }
}

impl std::ops::Deref for IdentifierList {
    type Target = Vec<Identifier>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for IdentifierList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", join_identifiers(&self.0, ", "))
    }
}

impl Parse for IdentifierList {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut vec = Vec::new();
        for pair in pair.inner() {
            if pair.as_rule() == Rule::identifier {
                vec.push(Identifier::parse(pair)?);
            }
        }
        Ok(Self(Refer::new(vec, pair.into())))
    }
}

impl std::iter::IntoIterator for IdentifierList {
    type Item = Identifier;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
