// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Named tuple type syntax element

use crate::{syntax::*, ty::*};

/// Named tuple (e.g. `(n: Scalar, m: String)`)
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TupleType(pub std::collections::BTreeMap<Identifier, Type>);

impl TupleType {
    /// Create new named tuple type.
    pub fn new(map: std::collections::BTreeMap<Identifier, Type>) -> Self {
        Self(map)
    }

    /// Create new Vec2 type.
    pub fn new_vec2() -> Self {
        [("x", Type::scalar()), ("y", Type::scalar())].iter().into()
    }

    /// Create new Vec3 type.
    pub fn new_vec3() -> Self {
        [
            ("x", Type::scalar()),
            ("y", Type::scalar()),
            ("z", Type::scalar()),
        ]
        .iter()
        .into()
    }

    /// Create new Color type.
    pub fn new_color() -> Self {
        [
            ("r", Type::scalar()),
            ("g", Type::scalar()),
            ("b", Type::scalar()),
            ("a", Type::scalar()),
        ]
        .iter()
        .into()
    }

    /// Test if the named tuple has exactly the number of keys.
    fn has_exact_keys(&self, keys: &[&str]) -> bool {
        if self.0.len() != keys.len() {
            return false;
        }

        for key in keys {
            if !self.0.contains_key(&Identifier::no_ref(key)) {
                return false;
            }
        }

        true
    }

    /// Checks if the named tuple type only holds scalar values.
    fn is_scalar_only(&self) -> bool {
        self.common_type().is_some_and(|ty| ty == Type::scalar())
    }

    /// Test if all fields have a common type.
    pub(crate) fn common_type(&self) -> Option<Type> {
        let types = self.0.values().cloned().collect::<Vec<_>>();
        if let Some(ty) = types.first() {
            if types[1..].iter().all(|t| t == ty) {
                return Some(ty.clone());
            }
        }
        None
    }

    /// Check if the named tuple is a [`Color`].
    pub(crate) fn is_color(&self) -> bool {
        self.is_scalar_only() && self.has_exact_keys(&["r", "g", "b", "a"])
    }

    /// Check if the named tuple is a [`Vec2`].
    pub(crate) fn is_vec2(&self) -> bool {
        self.is_scalar_only() && self.has_exact_keys(&["x", "y"])
    }

    /// Check if the named tuple is a [`Vec3`].
    pub(crate) fn is_vec3(&self) -> bool {
        self.is_scalar_only() && self.has_exact_keys(&["x", "y", "z"])
    }
}

impl From<std::slice::Iter<'_, (&str, Type)>> for TupleType {
    fn from(values: std::slice::Iter<'_, (&str, Type)>) -> Self {
        Self::new(
            values
                .map(|(k, v)| (Identifier::no_ref(k), v.clone()))
                .collect(),
        )
    }
}

impl std::fmt::Display for TupleType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_color() {
            return write!(f, "Color");
        }
        if self.is_vec2() {
            return write!(f, "Vec2");
        }
        if self.is_vec3() {
            return write!(f, "Vec3");
        }

        write!(f, "(")?;
        for (i, (identifier, ty)) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{identifier}: {ty}")?;
        }
        write!(f, ")")
    }
}
