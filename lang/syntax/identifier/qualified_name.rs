// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{src_ref::*, syntax::*};

/// A qualifier name consists of a . separated list of identifiers
/// e.g. `a::b::c`
#[derive(Debug, Default, Clone, PartialEq, Hash, Eq, Ord, PartialOrd)]
pub struct QualifiedName(Refer<Vec<Identifier>>);

/// List of qualified names which can pe displayed
#[derive(Debug)]
pub struct QualifiedNames(Vec<QualifiedName>);

impl std::fmt::Display for QualifiedNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|name| name.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl std::ops::Deref for QualifiedNames {
    type Target = Vec<QualifiedName>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromIterator<QualifiedName> for QualifiedNames {
    fn from_iter<T: IntoIterator<Item = QualifiedName>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl QualifiedName {
    /// Create [`QualifiedName`] from [`identifier`]s.
    ///
    /// - `ids`: *Identifiers* that concatenate to the *qualified name*.
    /// - `src_ref`: Reference for the whole name.
    pub fn new(ids: Vec<Identifier>, src_ref: SrcRef) -> Self {
        Self(Refer::new(ids, src_ref))
    }

    /// Create *qualified name* from [`identifier`]s without source code reference.
    ///
    /// - `ids`: *Identifiers* that concatenate to the *qualified name*.
    pub fn no_ref(ids: Vec<Identifier>) -> Self {
        Self(Refer::none(ids))
    }

    /// If the QualifiedName only consists of a single identifier, return it
    pub fn single_identifier(&self) -> Option<&Identifier> {
        if self.0.len() == 1 {
            self.0.first()
        } else {
            None
        }
    }

    /// Returns true if self is a qualified name with multiple ids in it
    pub fn is_qualified(&self) -> bool {
        self.0.len() > 1
    }

    /// Returns true if name contains exactly one id
    pub fn is_id(&self) -> bool {
        self.0.len() == 1
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
        Self(Refer::new(self.0[1..].to_vec(), self.0.src_ref.clone()))
    }

    /// remove the first name from path
    pub fn remove_last(&self) -> Self {
        Self(Refer::new(
            self.0[..self.0.len() - 1].to_vec(),
            self.0.src_ref.clone(),
        ))
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
            _ => (self.0[0].clone(), Self(Refer::none(self.0[1..].into()))),
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

    /// Add given prefix to name
    pub fn with_prefix(&self, prefix: &QualifiedName) -> Self {
        let mut full_name = prefix.clone();
        full_name.append(&mut self.clone());
        full_name
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
        if self.is_empty() {
            write!(f, "<none>")
        } else {
            write!(f, "{}", join_identifiers(&self.0, "::"))
        }
    }
}

impl SrcReferrer for QualifiedName {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

impl From<Refer<Vec<Identifier>>> for QualifiedName {
    fn from(value: Refer<Vec<Identifier>>) -> Self {
        Self(value)
    }
}

impl From<&Identifier> for QualifiedName {
    fn from(id: &Identifier) -> Self {
        Self(Refer::none(vec![id.clone()]))
    }
}

#[cfg(test)]
impl From<&str> for QualifiedName {
    fn from(value: &str) -> Self {
        Self(Refer::none(
            value.split("::").map(Identifier::from).collect(),
        ))
    }
}

#[cfg(not(test))]
impl TryFrom<&str> for QualifiedName {
    type Error = crate::parse::ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut name = Vec::new();
        for id in value.split("::").map(Identifier::try_from) {
            if id.is_err() {
                return Err(crate::parse::ParseError::InvalidQualifiedName(value.into()));
            }
            name.push(id.expect("unexpected error"));
        }

        Ok(Self(Refer::none(name)))
    }
}

impl TryFrom<String> for QualifiedName {
    type Error = crate::parse::ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut name = Vec::new();
        for id in value.split("::").map(Identifier::try_from) {
            if id.is_err() {
                return Err(crate::parse::ParseError::InvalidQualifiedName(value));
            }
            name.push(id.expect("unexpected error"));
        }

        Ok(Self(Refer::none(name)))
    }
}

impl From<Identifier> for QualifiedName {
    fn from(id: Identifier) -> Self {
        let src_ref = id.src_ref();
        QualifiedName(Refer::new(vec![id], src_ref))
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
