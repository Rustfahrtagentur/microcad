// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{src_ref::*, syntax::*};
use derive_more::{Deref, DerefMut};

/// A *qualified name* consists of a list of *identifiers*, separated by `::`,
/// e.g. `a::b::c`
#[derive(Default, Clone, PartialEq, Hash, Eq, Ord, PartialOrd, DerefMut, Deref)]
pub struct QualifiedName(Refer<Vec<Identifier>>);

/// List of *qualified names* which can be displayed.
#[derive(Deref)]
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

impl std::fmt::Debug for QualifiedNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|name| format!("{:?}", name.to_string()))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

pub(crate) type QualifiedNameSet = indexmap::IndexSet<QualifiedName>;

impl FromIterator<QualifiedName> for QualifiedNames {
    fn from_iter<T: IntoIterator<Item = QualifiedName>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl QualifiedName {
    /// Create [`QualifiedName`] from [`Identifier`]s.
    ///
    /// - `ids`: *Identifiers* that concatenate to the *qualified name*.
    /// - `src_ref`: Reference for the whole name.
    pub fn new(ids: Vec<Identifier>, src_ref: SrcRef) -> Self {
        Self(Refer::new(ids, src_ref))
    }

    /// Create [`QualifiedName`] from single [`Identifier`].
    pub fn from_id(id: Identifier) -> Self {
        let src_ref = id.src_ref();
        Self(Refer::new(vec![id], src_ref))
    }

    /// Create *qualified name* from [`identifier`]s without source code reference.
    ///
    /// - `ids`: *Identifiers* that concatenate to the *qualified name*.
    pub fn no_ref(ids: Vec<Identifier>) -> Self {
        Self(Refer::none(ids))
    }

    /// If the QualifiedName only consists of a single identifier, return it
    pub fn single_identifier(&self) -> Option<&Identifier> {
        if self.is_single_identifier() {
            self.0.first()
        } else {
            None
        }
    }

    /// Returns true if the QualifiedName only consists of a single identifier.
    pub fn is_single_identifier(&self) -> bool {
        self.0.len() == 1
    }

    /// Returns true if self is a qualified name with multiple ids in it
    pub fn is_qualified(&self) -> bool {
        self.0.len() > 1
    }

    /// Returns true if name contains exactly one id
    pub fn is_id(&self) -> bool {
        self.0.len() == 1
    }

    /// Tells if self is in a specified module
    pub fn is_within(&self, module: &QualifiedName) -> bool {
        self.starts_with(module)
    }

    /// Returns `true` if this name is in builtin module
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
    pub fn remove_last(self) -> Self {
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

    /// Return the base of the given relative name.
    pub fn base(&self, relative: &Self) -> Self {
        if self == relative {
            QualifiedName::default()
        } else {
            assert!(!relative.is_empty());
            assert!(self.len() > relative.len());
            assert!(self.ends_with(relative));
            let (base, _) = self.split_at(self.len() - relative.len());
            base.iter().cloned().collect()
        }
    }

    /// Add given prefix to name
    pub fn with_prefix(&self, prefix: &QualifiedName) -> Self {
        let mut full_name = prefix.clone();
        full_name.append(&mut self.clone());
        full_name
    }

    /// Add a given identifier as suffix.
    pub fn with_suffix(&self, suffix: &Identifier) -> Self {
        let mut name = self.clone();
        name.push(suffix.clone());
        name
    }

    pub(crate) fn count_super(&self) -> usize {
        self.iter().take_while(|id| id.is_super()).count()
    }

    pub(crate) fn un_super(&self) -> Self {
        self.iter().filter(|id| !id.is_super()).cloned().collect()
    }
}

#[test]
fn test_base() {
    let d: QualifiedName = "a::b::c::d".into();
    assert_eq!(d.base(&"b::c::d".into()), "a".into());
    assert_eq!(d.base(&"c::d".into()), "a::b".into());
    assert_eq!(d.base(&"d".into()), "a::b::c".into());
}

#[test]
#[should_panic]
fn test_base_panic() {
    let d: QualifiedName = "a::b::c::d".into();
    assert_eq!(d.base(&"a::b::c::d".into()), "".into());
}

#[test]
fn dissolve_super() {
    let what: QualifiedName = "super::super::c::x".into();
    assert_eq!(what.count_super(), 2);
}

impl std::fmt::Display for QualifiedName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, crate::invalid_no_ansi!(NAME))
        } else {
            write!(
                f,
                "{}",
                self.iter()
                    .map(|id| format!("{id:?}"))
                    .collect::<Vec<_>>()
                    .join("::")
            )
        }
    }
}

impl std::fmt::Debug for QualifiedName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, crate::invalid!(NAME))
        } else {
            write!(
                f,
                "{}",
                self.iter()
                    .map(|id| format!("{id:?}"))
                    .collect::<Vec<_>>()
                    .join("::")
            )
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

impl FromIterator<Identifier> for QualifiedName {
    fn from_iter<T: IntoIterator<Item = Identifier>>(iter: T) -> Self {
        Self(Refer::none(iter.into_iter().collect()))
    }
}

impl From<&Identifier> for QualifiedName {
    fn from(id: &Identifier) -> Self {
        Self(Refer::none(vec![id.clone()]))
    }
}

impl From<&std::path::Path> for QualifiedName {
    fn from(path: &std::path::Path) -> Self {
        // check if this is a module file and remove doublet module generation
        let path = if path.file_stem() == Some(std::ffi::OsStr::new("mod")) {
            path.parent().expect("mod file in root path is not allowed")
        } else {
            path
        };

        QualifiedName::no_ref(
            path.iter()
                .map(|id| {
                    Identifier(Refer {
                        value: id.to_string_lossy().into_owned().into(),
                        src_ref: SrcRef(None),
                    })
                })
                .collect(),
        )
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
        value
            .iter()
            .map(|id| format!("{id}"))
            .collect::<Vec<_>>()
            .join("::")
    }
}

impl TreeDisplay for QualifiedName {
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result {
        writeln!(
            f,
            "{:depth$}QualifiedName: '{}'",
            "",
            self.iter()
                .map(|id| format!("{id:?}"))
                .collect::<Vec<_>>()
                .join("::")
        )
    }
}
