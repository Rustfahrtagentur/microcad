// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Name list for collecting names and locals from within definitions.

use crate::syntax::*;

#[derive(Default)]
pub(crate) struct NameList {
    symbols: QualifiedNameSet,
    locals: IdentifierSet,
}

impl NameList {
    pub(crate) fn iter(&self) -> indexmap::set::Iter<'_, QualifiedName> {
        self.symbols.iter()
    }

    pub(super) fn drop_locals(self) -> Self {
        Self {
            symbols: self.symbols,
            ..Default::default()
        }
    }

    pub(super) fn add_names<'a>(mut self, iter: impl Iterator<Item = &'a QualifiedName>) -> Self {
        self.symbols.extend(iter.cloned());
        self
    }

    pub(super) fn add_name(self, name: &QualifiedName) -> Self {
        self.add_names(std::iter::once(name))
    }

    pub(super) fn add_as_name(self, id: &Identifier) -> Self {
        self.add_names(std::iter::once(&QualifiedName::from_id(id.clone())))
    }

    pub(super) fn add_locals<'a>(mut self, iter: impl Iterator<Item = &'a Identifier>) -> Self {
        self.locals.extend(iter.cloned());
        self
    }

    pub(super) fn add_local(self, id: &Identifier) -> Self {
        self.add_locals(std::iter::once(id))
    }

    pub(super) fn merge(mut self, other: NameList) -> Self {
        self.merge_in_place(other);
        self
    }

    pub(super) fn merge_many(self, mut iter: impl Iterator<Item = NameList>) -> Self {
        if let Some(next) = iter.next() {
            self.merge(next).merge_many(iter)
        } else {
            self
        }
    }

    pub(super) fn merge_in_place(&mut self, other: NameList) {
        self.symbols
            .extend(other.symbols.into_iter().filter(|name| {
                if let Some(id) = name.single_identifier() {
                    !self.locals.contains(id)
                } else {
                    true
                }
            }));
        self.locals.extend(other.locals.iter().cloned());
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }
}

impl From<Vec<&QualifiedName>> for NameList {
    fn from(names: Vec<&QualifiedName>) -> Self {
        NameList::from_iter(names)
    }
}

impl From<&QualifiedName> for NameList {
    fn from(name: &QualifiedName) -> Self {
        NameList::from_iter(std::iter::once(name))
    }
}

impl From<Vec<&Identifier>> for NameList {
    fn from(ids: Vec<&Identifier>) -> Self {
        NameList::from_iter(ids)
    }
}

impl From<&Identifier> for NameList {
    fn from(id: &Identifier) -> Self {
        NameList::from_iter(std::iter::once(id))
    }
}

impl<'a> FromIterator<&'a QualifiedName> for NameList {
    fn from_iter<T: IntoIterator<Item = &'a QualifiedName>>(iter: T) -> Self {
        Self {
            symbols: indexmap::IndexSet::from_iter(iter.into_iter().cloned()),
            ..Default::default()
        }
    }
}

impl<'a> FromIterator<&'a Identifier> for NameList {
    fn from_iter<T: IntoIterator<Item = &'a Identifier>>(iter: T) -> Self {
        Self {
            locals: IdentifierSet::from_iter(iter.into_iter().cloned()),
            ..Default::default()
        }
    }
}

impl std::fmt::Display for NameList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.symbols
                .iter()
                .map(|name| name.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl std::fmt::Debug for NameList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.symbols
            .iter()
            .try_for_each(|name| writeln!(f, "  {name} at {:?}", name.src_ref))
    }
}
