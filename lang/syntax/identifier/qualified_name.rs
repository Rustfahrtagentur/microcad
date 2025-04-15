// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, src_ref::*, syntax::*, Id};

/// A qualifier name consists of a . separated list of identifiers
/// e.g. `a::b::c`
#[derive(Debug, Default, Clone, PartialEq, Hash, Eq, Ord, PartialOrd)]
pub struct QualifiedName(pub Vec<Identifier>);

impl QualifiedName {
    /// If the QualifiedName only consists of a single identifier, return it
    pub fn single_identifier(&self) -> Option<&Identifier> {
        if self.0.len() == 1 {
            self.0.first()
        } else {
            None
        }
    }
    /// Tells if self is in a specified namespace
    pub fn is_sub_of(&self, namespace: &QualifiedName) -> bool {
        self.starts_with(namespace)
    }

    /// Returns `true` if this name is in builtin namespace
    pub fn is_builtin(&self) -> bool {
        if let Some(first) = self.first() {
            first == "__builtin"
        } else {
            false
        }
    }

    /// remove the first name from path
    pub fn remove_first(&self) -> Self {
        Self(self.0[1..].to_vec())
    }

    /// remove the first name from path
    pub fn remove_last(&self) -> Self {
        Self(self.0[..self.0.len() - 1].to_vec())
    }

    /// Append identifier to name
    pub fn push(&mut self, id: Identifier) {
        self.0.push(id)
    }

    /// Split name into first id and the rest
    pub fn split_first(&self) -> (Identifier, QualifiedName) {
        match self.len() {
            0 => todo!("return None or error?"),
            1 => (self.0[0].clone(), Self::default()),
            _ => (self.0[0].clone(), Self(self.0[1..].into())),
        }
    }

    /// return basename, `std::geo2d` returns `std`
    pub fn basename(&self) -> Option<Self> {
        let mut s = self.clone();
        if s.len() >= 2 {
            s.pop();
            Some(s)
        } else {
            None
        }
    }
}

impl SrcReferrer for QualifiedName {
    fn src_ref(&self) -> SrcRef {
        SrcRef::from_vec(&self.0)
    }
}

impl std::ops::Deref for QualifiedName {
    type Target = Vec<Identifier>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for QualifiedName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for QualifiedName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", join_identifiers(&self.0, "::"))
    }
}

#[cfg(test)]
impl From<&str> for QualifiedName {
    fn from(value: &str) -> Self {
        Self(value.split("::").map(Identifier::from).collect())
    }
}

#[cfg(not(test))]
impl TryFrom<&str> for QualifiedName {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut name = Vec::new();
        for id in value.split("::").map(Identifier::try_from) {
            if id.is_err() {
                return Err(ParseError::InvalidQualifiedName(value.into()));
            }
            name.push(id.expect("unexpected error"));
        }

        Ok(Self(name))
    }
}

impl From<Identifier> for QualifiedName {
    fn from(id: Identifier) -> Self {
        QualifiedName(vec![id])
    }
}

impl From<QualifiedName> for String {
    fn from(value: QualifiedName) -> Self {
        join_identifiers(&value.0, "::")
    }
}

impl PrintSyntax for QualifiedName {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(
            f,
            "{:depth$}QualifiedName: '{}'",
            "",
            join_identifiers(&self.0, "::")
        )
    }
}

impl TryFrom<QualifiedName> for Id {
    type Error = ParseError;

    fn try_from(qualified_name: QualifiedName) -> Result<Self, Self::Error> {
        match qualified_name.as_slice() {
            [identifier] => Ok(identifier.id().clone()),
            _ => Err(ParseError::QualifiedNameIsNoId(qualified_name)),
        }
    }
}
