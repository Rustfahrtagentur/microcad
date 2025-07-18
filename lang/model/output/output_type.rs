// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Export attribute.

/// The output type of the [`Model`].
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum OutputType {
    /// The output type has not yet been determined.
    #[default]
    NotDetermined,
    /// The [`Model`] outputs a 2d geometry.
    Geometry2D,
    /// The [`Model`] outputs a 3d geometry.
    Geometry3D,
    /// The [`Model`] is invalid, you cannot mix 2d and 3d geometry.
    InvalidMixed,
}

impl OutputType {
    /// Merge this output type with another.
    pub fn merge(&self, other: &Self) -> OutputType {
        match (self, other) {
            (OutputType::NotDetermined, output_type) => *output_type,
            (OutputType::Geometry2D, OutputType::NotDetermined)
            | (OutputType::Geometry2D, OutputType::Geometry2D)
            | (OutputType::Geometry3D, OutputType::NotDetermined)
            | (OutputType::Geometry3D, OutputType::Geometry3D) => *self,
            (OutputType::Geometry2D, OutputType::Geometry3D)
            | (OutputType::Geometry3D, OutputType::Geometry2D)
            | (OutputType::Geometry2D, OutputType::InvalidMixed)
            | (OutputType::Geometry3D, OutputType::InvalidMixed)
            | (OutputType::InvalidMixed, _) => OutputType::InvalidMixed,
        }
    }
}

impl std::fmt::Display for OutputType {
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
