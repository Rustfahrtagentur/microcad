// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Named tuple evaluation entity

use crate::{ty::*, value::*};

/// Tuple with named values
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Tuple(std::collections::BTreeMap<Identifier, Value>);

impl Tuple {
    /// Create new named tuple instance.
    pub fn new(map: std::collections::BTreeMap<Identifier, Value>) -> Self {
        Self(map)
    }

    /// Create a new named tuple from a slice of values.
    ///
    /// This function is used to create named tuples from built-in types like `Vec3` and `Color`.
    pub fn new_from_slice<T: Into<Value> + Copy>(values: &[(&'static str, T)]) -> Self {
        Self::new(
            values
                .iter()
                .map(|(k, v)| (Identifier::no_ref(k), (*v).into()))
                .collect(),
        )
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

impl From<Vec2> for Tuple {
    fn from(v: Vec2) -> Self {
        Self::new_from_slice(&[("x", v.x), ("y", v.y)])
    }
}

impl From<Vec3> for Tuple {
    fn from(v: Vec3) -> Self {
        Self::new_from_slice(&[("x", v.x), ("y", v.y), ("z", v.z)])
    }
}

impl From<Color> for Tuple {
    fn from(color: Color) -> Self {
        Self::new_from_slice(&[
            ("r", color.r),
            ("g", color.g),
            ("b", color.b),
            ("a", color.a),
        ])
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
