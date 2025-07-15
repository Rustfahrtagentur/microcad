// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tuple type syntax element

use crate::{syntax::*, ty::*};

/// (Partially named) tuple (e.g. `(n: Scalar, m: String. Integer)`)
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TupleType {
    pub(crate) named: std::collections::HashMap<Identifier, Type>,
    pub(crate) unnamed: std::collections::HashSet<Type>,
}
impl TupleType {
    /// Create new Vec2 type.
    pub fn new_vec2() -> Self {
        [("x", Type::scalar()), ("y", Type::scalar())]
            .into_iter()
            .collect()
    }

    /// Create new Vec3 type.
    pub fn new_vec3() -> Self {
        [
            ("x", Type::scalar()),
            ("y", Type::scalar()),
            ("z", Type::scalar()),
        ]
        .into_iter()
        .collect()
    }

    /// Create new Color type.
    pub fn new_color() -> Self {
        [
            ("r", Type::scalar()),
            ("g", Type::scalar()),
            ("b", Type::scalar()),
            ("a", Type::scalar()),
        ]
        .into_iter()
        .collect()
    }

    /// Create new Size2D type.
    pub fn new_size2d() -> Self {
        [("width", Type::length()), ("height", Type::length())]
            .into_iter()
            .collect()
    }

    /// Test if the named tuple has exactly all the given keys
    fn matches(&self, keys: &[&str]) -> bool {
        if !self.unnamed.is_empty() && self.named.len() != keys.len() {
            return false;
        }
        keys.iter()
            .all(|k| self.named.contains_key(&Identifier::no_ref(k)))
    }

    /// Checks if the named tuple type only holds scalar values.
    fn is_scalar_only(&self) -> bool {
        self.common_type().is_some_and(|ty| *ty == Type::scalar())
    }

    /// Checks if the named tuple type only holds length values.
    fn is_length_only(&self) -> bool {
        self.common_type().is_some_and(|ty| *ty == Type::length())
    }

    /// Test if all fields have a common type.
    pub(crate) fn common_type(&self) -> Option<&Type> {
        let mut iter = self.unnamed.iter().chain(self.named.values());
        if let Some(first) = iter.next() {
            if iter.all(|x| x == first) {
                return Some(first);
            }
        }
        None
    }

    /// Check if the named tuple is a [`Color`].
    pub(crate) fn is_color(&self) -> bool {
        self.is_scalar_only() && self.matches(&["r", "g", "b", "a"])
    }

    /// Check if the named tuple is a [`Vec2`].
    pub(crate) fn is_vec2(&self) -> bool {
        self.is_scalar_only() && self.matches(&["x", "y"])
    }

    /// Check if the named tuple is a [`Vec3`].
    pub(crate) fn is_vec3(&self) -> bool {
        self.is_scalar_only() && self.matches(&["x", "y", "z"])
    }

    /// Check if the named tuple is a [`Size2D`]
    pub(crate) fn is_size2d(&self) -> bool {
        self.is_length_only() && self.matches(&["width", "height"])
    }
}

impl std::hash::Hash for TupleType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.named.iter().for_each(|(id, ty)| {
            id.hash(state);
            ty.hash(state)
        });
        self.unnamed.iter().for_each(|ty| ty.hash(state));
    }
}

impl FromIterator<(Identifier, Type)> for TupleType {
    fn from_iter<T: IntoIterator<Item = (Identifier, Type)>>(iter: T) -> Self {
        let (unnamed, named) = iter.into_iter().partition(|(id, _)| id.is_empty());
        Self {
            named,
            unnamed: unnamed.into_values().collect(),
        }
    }
}

impl<'a> FromIterator<(&'a str, Type)> for TupleType {
    fn from_iter<T: IntoIterator<Item = (&'a str, Type)>>(iter: T) -> Self {
        let (unnamed, named) = iter
            .into_iter()
            .map(|(id, ty)| (Identifier::no_ref(id), ty))
            .partition(|(id, _)| id.is_empty());
        Self {
            named,
            unnamed: unnamed.into_values().collect(),
        }
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
        if self.is_size2d() {
            return write!(f, "Size2D");
        }

        write!(f, "({})", {
            let mut types = self
                .named
                .iter()
                .map(|(id, ty)| format!("{id}: {ty}"))
                .chain(self.unnamed.iter().map(|ty| ty.to_string()))
                .collect::<Vec<_>>();

            types.sort();
            types.join(", ")
        })
    }
}
