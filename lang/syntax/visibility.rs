// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Visibility of an entity.

/// Visibility of an entity.
///
/// This is used to determine if an entity is public or private.
/// By default, entities are private.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum Visibility {
    /// Private visibility
    #[default]
    Private,
    /// Public visibility
    Public,
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Visibility::Private => Ok(()),
            Visibility::Public => write!(f, "pub "),
        }
    }
}
