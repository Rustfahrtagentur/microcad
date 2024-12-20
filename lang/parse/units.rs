// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad unit parser entity

use crate::{parse::*, parser::*, r#type::*};

/// The units that can be used after numbers in the language"
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Unit {
    /// No unit was given
    None,
    /// Percents
    Percent,

    /// Meters
    M,
    /// Centimeters
    Cm,
    /// Millimeters
    Mm,
    /// Micrometers
    Micrometer,
    /// Inches
    In,

    /// Degree
    Deg,
    /// Degree
    DegS,
    /// Gradient
    Grad,
    /// Turns
    Turn,
    /// Radians
    Rad,

    /// Grams
    G,
    /// Kilograms
    Kg,
    /// Pounds
    Lb,

    /// Square Centimeters
    Cm2,
    /// Square Millimeters
    Mm2,
    /// Square Inches
    In2,
    /// Square Meters
    M2,

    /// Cubic Centimeters
    Cm3,
    /// Cubic Millimeters
    Mm3,
    /// Cubic Inches
    In3,
    /// Cubic Meters
    M3,
    /// Liters
    Liter,
    /// Centiliter
    Centiliter,
    ///Milliliter
    Milliliter,
    /// Microliter
    Microliter,
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // Scalars
            Self::None => write!(f, ""),
            Self::Percent => write!(f, "%"),

            // Lengths
            Self::M => write!(f, "m"),
            Self::Cm => write!(f, "cm"),
            Self::Mm => write!(f, "mm"),
            Self::Micrometer => write!(f, "µm"),
            Self::In => write!(f, "in"),

            // Angles
            Self::Deg => write!(f, "deg"),
            Self::DegS => write!(f, "°"),
            Self::Grad => write!(f, "grad"),
            Self::Turn => write!(f, "turn"),
            Self::Rad => write!(f, "rad"),

            // Weights
            Self::G => write!(f, "g"),
            Self::Kg => write!(f, "kg"),
            Self::Lb => write!(f, "lb"),

            // Areas
            Self::Mm2 => write!(f, "mm²"),
            Self::Cm2 => write!(f, "cm²"),
            Self::M2 => write!(f, "m³"),
            Self::In2 => write!(f, "in²"),

            // Volumes
            Self::Mm3 => write!(f, "mm³"),
            Self::Cm3 => write!(f, "cm³"),
            Self::M3 => write!(f, "m³"),
            Self::In3 => write!(f, "in³"),
            Self::Milliliter => write!(f, "ml"),
            Self::Centiliter => write!(f, "cl"),
            Self::Liter => write!(f, "l"),
            Self::Microliter => write!(f, "µl"),
        }
    }
}
impl std::str::FromStr for Unit {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // Scalars
            "" => Ok(Self::None),
            "%" => Ok(Self::Percent),

            // Lengths
            "m" => Ok(Self::M),
            "cm" => Ok(Self::Cm),
            "mm" => Ok(Self::Mm),
            "µm" => Ok(Self::Micrometer),
            "in" => Ok(Self::In),

            // Angles
            "deg" => Ok(Self::Deg),
            "°" => Ok(Self::DegS),
            "grad" => Ok(Self::Grad),
            "turn" => Ok(Self::Turn),
            "rad" => Ok(Self::Rad),

            // Weights
            "g" => Ok(Self::G),
            "kg" => Ok(Self::Kg),
            "lb" => Ok(Self::Lb),

            // Areas
            "cm²" => Ok(Self::Cm2),
            "mm²" => Ok(Self::Mm2),
            "in²" => Ok(Self::In2),
            "m²" => Ok(Self::M2),

            // Volumes
            "cm³" => Ok(Self::Cm3),
            "mm³" => Ok(Self::Mm3),
            "in³" => Ok(Self::In3),
            "m³" => Ok(Self::M3),
            "ml" => Ok(Self::Milliliter),
            "cl" => Ok(Self::Centiliter),
            "l" => Ok(Self::Liter),
            "µl" => Ok(Self::Microliter),

            // Unknown
            _ => Err(ParseError::UnknownUnit(s.to_string())),
        }
    }
}
impl Unit {
    /// Return type to use with this unit
    pub fn ty(self) -> Type {
        match self {
            Self::None | Self::Percent => Type::Scalar,
            Self::M | Self::Cm | Self::Mm | Self::Micrometer | Self::In => Type::Length,
            Self::Deg | Self::DegS | Self::Grad | Self::Turn | Self::Rad => Type::Angle,
            Self::G | Self::Kg | Self::Lb => Type::Weight,
            Self::Mm2 | Self::Cm2 | Self::M2 | Self::In2 => Type::Area,
            Self::Mm3
            | Self::Cm3
            | Self::M3
            | Self::In3
            | Self::Liter
            | Self::Centiliter
            | Self::Milliliter
            | Self::Microliter => Type::Volume,
        }
    }
    /// Normalize value to mm, rad or gram
    pub fn normalize(self, x: f64) -> f64 {
        match self {
            // Scalar
            Self::None => x,
            Self::Percent => x * 0.01_f64,

            // Lengths
            Self::M => x * 1_000.0_f64,
            Self::Cm => x * 10.0_f64,
            Self::Mm => x,
            Self::Micrometer => x / 1_000.0_f64,
            Self::In => x * 25.4_f64,

            // Angles
            Self::Deg | Self::DegS => x / 180. * std::f64::consts::PI,
            Self::Grad => x / 200. * std::f64::consts::PI,
            Self::Turn => x * 2.0 * std::f64::consts::PI,
            Self::Rad => x,

            // Weights
            Self::G => x,
            Self::Kg => x * 1_000.0_f64,
            Self::Lb => x * 453.59237_f64,

            // Areas
            Self::Mm2 => x,
            Self::Cm2 => x * 100.0_f64,
            Self::M2 => x * 1_000_000.0_f64,
            Self::In2 => x * 645.16_f64,

            // Volumes
            Self::Mm3 => x,
            Self::Cm3 => x * 1_000.0_f64,
            Self::M3 => x * 1_000_000_000.0_f64,
            Self::In3 => x * 16387.06_f64,
            Self::Liter => x * 1_000_000.0_f64,
            Self::Centiliter => x * 100.0_f64,
            Self::Milliliter => x * 1_000.0_f64,
            Self::Microliter => x * 1_000_000.0_f64,
        }
    }
}

impl Parse for Unit {
    fn parse(pair: Pair) -> ParseResult<Self> {
        use std::str::FromStr;
        match Unit::from_str(pair.as_str()) {
            Ok(unit) => Ok(unit),
            Err(_) => Err(ParseError::UnknownUnit(pair.as_str().to_string())),
        }
    }
}
