// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Named tuple value

mod error;

pub use error::*;

use crate::{ty::*, value::*};

/// Tuple with named values
#[derive(Clone, Debug, PartialEq, Default)]
pub struct NamedTuple(std::collections::BTreeMap<Identifier, Value>);

impl NamedTuple {
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

    /// Fetch a tuple field by name as `&str`.
    ///
    /// This method does not provide error handling and is supposed to be used for built-ins.
    pub fn get<'a, T>(&'a self, id: &str) -> Option<T>
    where
        T: std::convert::TryFrom<&'a Value>,
        T::Error: std::fmt::Debug,
    {
        self.get_by_id(&Identifier::no_ref(id))
            .map(|v| v.try_into().expect("cannot convert value"))
    }

    /// Fetch a tuple field by name as `&str`.
    pub fn get_by_id(&self, id: &Identifier) -> Option<&Value> {
        self.0.get(id)
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

impl From<Vec2> for NamedTuple {
    fn from(v: Vec2) -> Self {
        Self::new_from_slice(&[("x", v.x), ("y", v.y)])
    }
}

impl From<Vec3> for NamedTuple {
    fn from(v: Vec3) -> Self {
        Self::new_from_slice(&[("x", v.x), ("y", v.y), ("z", v.z)])
    }
}

impl From<Color> for NamedTuple {
    fn from(color: Color) -> Self {
        Self::new_from_slice(&[
            ("r", color.r),
            ("g", color.g),
            ("b", color.b),
            ("a", color.a),
        ])
    }
}

impl<'a> TryFrom<&'a Value> for &'a NamedTuple {
    type Error = ValueError;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::NamedTuple(named_tuple) => Ok(named_tuple),
            _ => Err(ValueError::CannotConvert(
                value.clone(),
                "NamedTuple".into(),
            )),
        }
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
