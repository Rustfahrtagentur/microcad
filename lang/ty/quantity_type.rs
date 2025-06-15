// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad quantity type

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
        let name: &'static str = self.into();
        write!(f, "{name}")
    }
}

impl std::ops::Mul for QuantityType {
    type Output = QuantityType;

    fn mul(self, rhs: Self) -> Self::Output {
        if self == Self::Invalid || rhs == Self::Invalid {
            return Self::Invalid;
        }
        if self == QuantityType::Scalar {
            return rhs;
        }
        if rhs == QuantityType::Scalar {
            return self;
        }

        match (self, rhs) {
            (QuantityType::Length, QuantityType::Length) => QuantityType::Area,
            (QuantityType::Length, QuantityType::Area)
            | (QuantityType::Area, QuantityType::Length) => QuantityType::Volume,
            (_, _) => QuantityType::Invalid,
        }
    }
}

impl std::ops::Div for QuantityType {
    type Output = QuantityType;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs == self {
            return QuantityType::Scalar;
        }
        if rhs == QuantityType::Scalar {
            return self;
        }

        match (self, rhs) {
            (QuantityType::Volume, QuantityType::Length) => QuantityType::Area,
            (QuantityType::Volume, QuantityType::Area) => QuantityType::Length,
            (_, _) => QuantityType::Invalid,
        }
    }
}
