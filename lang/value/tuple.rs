// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Named tuple evaluation entity

use crate::{ty::*, value::*};

/// Tuple with named values
///
/// Names are optional, which means Identifiers can be empty.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Tuple(std::collections::BTreeMap<Identifier, Value>);

impl Tuple {
    /// Create new named tuple instance.
    pub fn new(map: std::collections::BTreeMap<Identifier, Value>) -> Self {
        Self(map)
    }
}

impl std::ops::Deref for Tuple {
    type Target = std::collections::BTreeMap<Identifier, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Tuple {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Into<Value> + Clone> From<std::slice::Iter<'_, (&'static str, T)>> for Tuple {
    fn from(values: std::slice::Iter<'_, (&'static str, T)>) -> Self {
        Self::new(
            values
                .map(|(k, v)| (Identifier::no_ref(k), (*v).clone().into()))
                .collect(),
        )
    }
}

impl From<Vec2> for Tuple {
    fn from(v: Vec2) -> Self {
        [("x", v.x), ("y", v.y)].iter().into()
    }
}

impl From<Vec3> for Tuple {
    fn from(v: Vec3) -> Self {
        [("x", v.x), ("y", v.y), ("z", v.z)].iter().into()
    }
}

impl From<Color> for Tuple {
    fn from(color: Color) -> Self {
        [
            ("r", color.r),
            ("g", color.g),
            ("b", color.b),
            ("a", color.a),
        ]
        .iter()
        .into()
    }
}

impl std::fmt::Display for Tuple {
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

impl crate::ty::Ty for Tuple {
    fn ty(&self) -> Type {
        Type::Tuple(TupleType(
            self.0
                .iter()
                .map(|(name, v)| (name.clone(), v.ty().clone()))
                .collect(),
        ))
    }
}
