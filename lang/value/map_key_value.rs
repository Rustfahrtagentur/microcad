// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Map key type evaluation entity

use crate::{syntax::*, value::*};
use microcad_core::Integer;

/// A value type that can be used as a key in a map
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum MapKeyValue {
    /// Integer map key type
    Integer(Integer),
    /// Boolean map key type
    Bool(bool),
    /// String map key type
    String(String),
}

impl MapKeyValue {
    /// Return the used key type
    /// TODO: This method seems not in any use and it looks like
    /// `Ty::ty()` but it isn't returning a `Type` but a MapKeyType
    pub fn ty(&self) -> MapKeyType {
        match self {
            MapKeyValue::Integer(_) => MapKeyType::Integer,
            MapKeyValue::Bool(_) => MapKeyType::Bool,
            MapKeyValue::String(_) => MapKeyType::String,
        }
    }
}

impl TryFrom<Value> for MapKeyValue {
    type Error = ValueError;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        match value {
            Value::Integer(n) => Ok(MapKeyValue::Integer(n)),
            Value::Bool(b) => Ok(MapKeyValue::Bool(b)),
            Value::String(s) => Ok(MapKeyValue::String(s)),
            value => Err(ValueError::InvalidMapKeyType(value.ty())),
        }
    }
}

impl std::fmt::Display for MapKeyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MapKeyValue::Integer(n) => write!(f, "{n}"),
            MapKeyValue::Bool(b) => write!(f, "{b}"),
            MapKeyValue::String(s) => write!(f, "{s}"),
        }
    }
}
