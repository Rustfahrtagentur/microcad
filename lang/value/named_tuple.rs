// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Named tuple evaluation entity

use crate::{ty::*, value::*};

/// Tuple with named values
#[derive(Clone, Debug, PartialEq)]
pub struct NamedTuple(std::collections::BTreeMap<Identifier, Value>);

impl NamedTuple {
    /// Create new named tuple instance.
    pub fn new(map: std::collections::BTreeMap<Identifier, Value>) -> Self {
        Self(map)
    }
}

impl std::ops::Deref for NamedTuple {
    type Target = std::collections::BTreeMap<Identifier, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for NamedTuple {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec3> for NamedTuple {
    fn from(value: Vec3) -> Self {
        NamedTuple::new(
            [("x", value.x), ("y", value.y), ("z", value.z)]
                .iter()
                .map(|(k, v)| (Identifier::no_ref(k), Value::Scalar(*v)))
                .collect(),
        )
    }
}

impl std::fmt::Display for NamedTuple {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({items})",
            items = self
                .0
                .iter()
                .map(|(k, v)| format!("{k} => {v}"))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl crate::ty::Ty for NamedTuple {
    fn ty(&self) -> Type {
        Type::NamedTuple(NamedTupleType(
            self.0
                .iter()
                .map(|(name, v)| (name.clone(), v.ty().clone()))
                .collect(),
        ))
    }
}
