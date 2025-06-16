// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Value list evaluation entity

use crate::{ty::*, value::*};

/// List of values
#[derive(Clone, Debug, Default)]
pub struct ValueList(Vec<Value>);

impl ValueList {
    /// Create new value list.
    pub fn new(list: Vec<Value>) -> Self {
        Self(list)
    }

    /// add unit to values of primitive types ([Scalar] or [Integer]).
    pub fn bundle_unit(self, unit: Unit) -> ValueResult<Self> {
        if unit == Unit::None {
            return Ok(self);
        }
        let mut values = Vec::new();
        for value in self.0 {
            values.push(value.bundle_unit(unit)?);
        }

        Ok(Self(values))
    }

    /// Return list with types of values.
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

impl IntoIterator for ValueList {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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
        ValueList(vec)
    }
}
