// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad identifier syntax elements

mod identifier_list;
mod qualified_name;

pub use identifier_list::*;
pub use qualified_name::*;

use crate::{parse::*, parser::Parser, src_ref::*, syntax::*, Id};

/// µcad identifier
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier(pub Refer<Id>);

impl Identifier {
    /// Make empty (invalid) id
    pub fn none() -> Self {
        Self(Refer::none("".into()))
    }

    /// Check if this was created with none()
    pub fn is_none(&self) -> bool {
        self.0.src_ref().is_empty() && self.0.is_empty()
    }

    /// Make empty (invalid) id
    pub fn no_ref(id: &str) -> Self {
        Self(Refer::none(id.into()))
    }

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

    /// check if this is a valid identifier (contains only `A`-`Z`, `a`-`z` or `_`)
    pub fn validate(self) -> ParseResult<Self> {
        Parser::parse_rule(crate::parser::Rule::identifier, self.id().as_str(), 0)
    }

    /// Add given `prefix` to identifier to get `qualified name`.
    pub fn with_prefix(&self, prefix: &QualifiedName) -> QualifiedName {
        QualifiedName::from(self).with_prefix(prefix)
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

impl std::str::FromStr for Identifier {
    type Err = crate::eval::EvalError;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        Ok(Identifier::no_ref(id).validate()?)
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
        Parser::parse_rule(crate::parser::Rule::identifier, value, 0)
    }
}

impl<'a> From<&'a Identifier> for &'a str {
    fn from(value: &'a Identifier) -> Self {
        &value.0
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, crate::invalid!(ID))
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl std::fmt::Debug for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Identifier: {:?}", self.0)
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
    let id1 = Identifier::no_ref("x");
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
