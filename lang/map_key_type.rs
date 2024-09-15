// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Type of a key in a `MapType`

use crate::{errors::*, r#type::*};

/// Key type for use in a `MapType`
#[derive(Debug, Default, Clone, PartialEq)]
pub enum MapKeyType {
    #[default]
    /// Integer value as key
    Integer,
    /// Boolean as key
    Bool,
    /// String as key
    String,
}

impl TryFrom<Type> for MapKeyType {
    type Error = ParseError;

    fn try_from(t: Type) -> Result<Self, Self::Error> {
        match t {
            Type::Integer => Ok(Self::Integer),
            Type::Bool => Ok(Self::Bool),
            Type::String => Ok(Self::String),
            _ => Err(ParseError::InvalidMapKeyType(t.to_string())),
        }
    }
}

impl std::fmt::Display for MapKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Integer => write!(f, "int"),
            Self::Bool => write!(f, "bool"),
            Self::String => write!(f, "string"),
        }
    }
}
