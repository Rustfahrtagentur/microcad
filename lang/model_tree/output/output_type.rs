// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Export attribute.

/// The output type of the [`ModelNode`].
#[derive(Debug, Clone, Default, PartialEq)]
pub enum ModelNodeOutputType {
    /// The output type has not yet been determined.
    #[default]
    NotDetermined,
    /// The [`ModelNode`] outputs a 2d geometry.
    Geometry2D,
    /// The [`ModelNode`] outputs a 3d geometry.
    Geometry3D,
    /// The [`ModelNode`] is invalid, you cannot mix 2d and 3d geometry.
    InvalidMixed,
}

impl std::fmt::Display for ModelNodeOutputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Self::NotDetermined => "<unknown>",
                Self::Geometry2D => "2D",
                Self::Geometry3D => "3D",
                Self::InvalidMixed => "<invalid>",
            }
        )
    }
}
