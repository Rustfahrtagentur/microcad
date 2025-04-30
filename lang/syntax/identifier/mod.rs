// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad identifier syntax elements

mod identifier_list;
mod qualified_name;

pub use identifier_list::*;
pub use qualified_name::*;

#[cfg(not(test))]
use crate::parse::*;
use crate::{Id, src_ref::*, syntax::*};

/// µcad identifier
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier(pub Refer<Id>);

impl Identifier {
    /// Get the value of the identifier
    pub fn id(&self) -> &Id {
        &self.0.value
    }

    /// Return number of identifiers in name
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Return if name is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl SrcReferrer for Identifier {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref.clone()
    }
}

impl std::hash::Hash for Identifier {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.0.hash(hasher)
    }
}

#[cfg(test)]
impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Self(Refer::none(value.into()))
    }
}

#[cfg(not(test))]
impl TryFrom<&str> for Identifier {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let id = Self(Refer::none(value.into()));
        if id.0.chars().all(|c| c.is_ascii_alphabetic() || c == '_') {
            Ok(id)
        } else {
            Err(ParseError::InvalidIdentifier(value.into()))
        }
    }
}

impl<'a> From<&'a Identifier> for &'a str {
    fn from(value: &'a Identifier) -> Self {
        &value.0
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<str> for Identifier {
    fn eq(&self, other: &str) -> bool {
        *self.0 == other
    }
}

impl PrintSyntax for Identifier {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}Identifier: {}", "", self.id())
    }
}

/// join several identifiers with `::` and return as string
pub fn join_identifiers(identifiers: &[Identifier], separator: &str) -> String {
    identifiers
        .iter()
        .map(|ident| ident.to_string())
        .collect::<Vec<_>>()
        .join(separator)
}

#[test]
fn identifier_comparison() {
    use crate::syntax::*;

    // same id but different src refs
    let id1 = Identifier(Refer::none("x".into()));
    let id2 = Identifier(Refer::new("x".into(), SrcRef::new(0..5, 0, 1, 1)));

    // shall be equal
    assert!(id1 == id2);
}

#[test]
fn identifier_hash() {
    use crate::syntax::*;
    use std::hash::{Hash, Hasher};

    // same id but different src refs
    let id1 = Identifier(Refer::none("x".into()));
    let id2 = Identifier(Refer::new("x".into(), SrcRef::new(0..5, 0, 1, 1)));

    let mut hasher = std::hash::DefaultHasher::new();
    id1.hash(&mut hasher);
    let hash1 = hasher.finish();
    let mut hasher = std::hash::DefaultHasher::new();
    id2.hash(&mut hasher);

    let hash2 = hasher.finish();

    // shall be equal
    assert_eq!(hash1, hash2);
}
