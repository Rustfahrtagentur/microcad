// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Value list evaluation entity

use crate::{src_ref::*, ty::*, value::*};

/// List of values
#[derive(Clone, Debug, Default)]
pub struct ValueList(Refer<Vec<Value>>);

impl ValueList {
    /// Create new value list
    pub fn new(list: Vec<Value>, src_ref: SrcRef) -> Self {
        Self(Refer::new(list, src_ref))
    }

    /// add unit to values of primitive types (Scalar or Integer)
    pub fn add_unit_to_unitless(&mut self, unit: Unit) -> std::result::Result<(), ValueError> {
        self.0
            .iter_mut()
            .try_for_each(|value| value.add_unit_to_unitless(unit))
    }

    /// Return list with types of values
    pub fn types(&self) -> TypeList {
        TypeList::new(
            self.0
                .iter()
                .map(|v| v.ty())
                .collect::<Vec<Type>>()
                .into_iter()
                .collect(),
        )
    }
}

impl std::ops::Deref for ValueList {
    type Target = Vec<Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ValueList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl SrcReferrer for ValueList {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

impl IntoIterator for ValueList {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.value.into_iter()
    }
}

impl PartialEq for ValueList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl std::iter::FromIterator<Value> for ValueList {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        let vec = Vec::from_iter(iter);
        let src_ref = SrcRef::from_vec(&vec);
        ValueList(Refer::new(vec, src_ref))
    }
}
