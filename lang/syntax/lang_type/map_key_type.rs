// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Type of a key in a `MapType`

/// Key type for use in a `MapType`
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum MapKeyType {
    #[default]
    /// Integer value as key
    Integer,
    /// Boolean as key
    Bool,
    /// String as key
    String,
}

impl std::fmt::Display for MapKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Integer => write!(f, "Int"),
            Self::Bool => write!(f, "Bool"),
            Self::String => write!(f, "String"),
        }
    }
}
