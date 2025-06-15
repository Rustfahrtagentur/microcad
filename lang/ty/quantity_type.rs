// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad quantity type

use compact_str::ToCompactString;
use strum::IntoStaticStr;

/// A quantity type with
#[derive(Clone, Debug, IntoStaticStr, PartialEq)]
pub enum QuantityType {
    /// A unitless scalar value.
    Scalar,
    /// Length in mm.
    Length,
    /// Area in mm².
    Area,
    /// Volume in mm³.
    Volume,
    /// Density in g/mm³
    Density,
    /// An angle in radians.
    Angle,
    /// Weight of a specific volume of material.
    Weight,
    /// An invalid, unsupported quantity type.
    Invalid,
}

impl std::fmt::Display for QuantityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_compact_string())
    }
}
