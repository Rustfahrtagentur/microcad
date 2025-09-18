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

static UNIQUE_ID_NEXT: std::sync::Mutex<usize> = std::sync::Mutex::new(0);

/// A case for an identifier.
#[derive(Debug, PartialEq, Eq)]
pub enum Case {
    /// PascalCase
    Pascal,
    /// lower_snake_case
    LowerSnake,
    /// UPPER_SNAKE_CASE
    UpperSnake,
    /// A
    UpperSingleChar,
    /// Invalid.
    Invalid,
}

impl Identifier {
    /// Make empty (invalid) id
    pub fn none() -> Self {
        Self(Refer::none("".into()))
    }

    /// Create new identifier with a new unique name.
    ///
    /// Every call will return a new identifier (which is a `$` followed by an counter)
    pub fn unique() -> Self {
        let mut num = UNIQUE_ID_NEXT
            .lock()
            .expect("lock on UNIQUE_ID_NEXT failed");
        let id = format!("${num}");
        *num += 1;
        Identifier::no_ref(&id)
    }

    /// Check if id is the `super` id
    pub fn is_super(&self) -> bool {
        *self.0 == "super"
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

    /// Detect if the identifier matches a certain case.
    pub fn detect_case(&self) -> Case {
        let s = &self.0.value;

        if s.is_empty() {
            return Case::Invalid;
        }

        if s.len() == 1 {
            let c = s.chars().next().expect("At least one char");
            if c.is_ascii_uppercase() {
                return Case::UpperSingleChar;
            } else {
                return Case::Invalid;
            }
        }

        let has_underscore = s.contains('_');

        if has_underscore {
            if s.chars().all(|c| c.is_ascii_uppercase() || c == '_') {
                return Case::UpperSnake;
            } else if s.chars().all(|c| c.is_ascii_lowercase() || c == '_') {
                return Case::LowerSnake;
            } else {
                return Case::Invalid;
            }
        } else {
            // Must be PascalCase: starts with uppercase and contains no underscores
            let mut chars = s.chars();
            if let Some(first) = chars.next() {
                if first.is_ascii_uppercase() && chars.all(|c| c.is_ascii_alphanumeric()) {
                    return Case::Pascal;
                }
            }
        }

        Case::Invalid
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
            write!(f, crate::invalid_no_ansi!(ID))
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl std::fmt::Debug for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "{}", crate::invalid!(ID))
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl PartialEq<str> for Identifier {
    fn eq(&self, other: &str) -> bool {
        *self.0 == other
    }
}

impl TreeDisplay for Identifier {
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}Identifier: {}", "", self.id())
    }
}

/// join several identifiers with `::` and return as string
pub fn join_identifiers(identifiers: &[Identifier], separator: &str) -> String {
    identifiers
        .iter()
        .map(|ident| format!("{ident}"))
        .collect::<Vec<_>>()
        .join(separator)
}

/// join several identifiers with `::` and return as string
pub fn join_identifiers_debug(identifiers: &[Identifier], separator: &str) -> String {
    identifiers
        .iter()
        .map(|ident| format!("{ident:?}"))
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

#[test]
fn identifier_case() {
    let detect_case = |s| -> Case { Identifier::no_ref(s).detect_case() };

    assert_eq!(detect_case("PascalCase"), Case::Pascal);
    assert_eq!(detect_case("lower_snake_case"), Case::LowerSnake);
    assert_eq!(detect_case("UPPER_SNAKE_CASE"), Case::UpperSnake);
    assert_eq!(detect_case("notValid123_"), Case::Invalid);
    assert_eq!(detect_case(""), Case::Invalid);
    assert_eq!(detect_case("A"), Case::UpperSingleChar); // New case
    assert_eq!(detect_case("z"), Case::Invalid); // lowercase single letter
    assert_eq!(detect_case("_"), Case::Invalid); // only underscore
    assert_eq!(detect_case("a_b"), Case::LowerSnake);
    assert_eq!(detect_case("A_B"), Case::UpperSnake);

    println!("All tests passed.");
}
