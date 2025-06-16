// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Named tuple type syntax element

use crate::{syntax::*, ty::*};

/// Named tuple (e.g. `(n: Scalar, m: String)`)
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NamedTupleType(pub std::collections::BTreeMap<Identifier, Type>);

impl NamedTupleType {
    /// Create new named tuple type.
    pub fn new(map: std::collections::BTreeMap<Identifier, Type>) -> Self {
        Self(map)
    }

    /// Create a new named tuple type from a slice of type.
    ///
    /// This function is used to create named tuples from built-in types like `Vec3` and `Color`.
    pub fn new_from_slice(values: &[(&'static str, Type)]) -> Self {
        Self::new(
            values
                .iter()
                .map(|(k, v)| (Identifier::no_ref(k), v.clone()))
                .collect(),
        )
    }

    /// Create new Vec2 type.
    pub fn new_vec2() -> Self {
        Self::new_from_slice(&[("x", Type::scalar()), ("y", Type::scalar())])
    }

    /// Create new Vec3 type.
    pub fn new_vec3() -> Self {
        Self::new_from_slice(&[
            ("x", Type::scalar()),
            ("y", Type::scalar()),
            ("z", Type::scalar()),
        ])
    }

    /// Create new Color type.
    pub fn new_color() -> Self {
        Self::new_from_slice(&[
            ("r", Type::scalar()),
            ("g", Type::scalar()),
            ("b", Type::scalar()),
            ("a", Type::scalar()),
        ])
    }
}

impl std::fmt::Display for NamedTupleType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
