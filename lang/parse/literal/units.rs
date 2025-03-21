// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad unit parser entity

use crate::{parse::*, parser::*, r#type::*};

/// The units that can be used after numbers in the language"
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Unit {
    // Scalar
    /// No unit was given
    None,
    /// Percents
    Percent,

    // Length
    /// Meters
    Meter,
    /// Centimeters
    Centimeter,
    /// Millimeters
    Millimeter,
    /// Micrometers
    Micrometer,
    /// Inches
    Inch,
    /// Feet
    Foot,
    /// Yards
    Yard,

    // Angle
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

    // Weight
    /// Grams
    Gram,
    /// Kilograms
    Kilogram,
    /// Pounds
    Pound,
    /// Ounces
    Ounce,

    // Areas
    /// Square Meters
    Meter2,
    /// Square Centimeters
    Centimeter2,
    /// Square Millimeters
    Millimeter2,
    /// Square Micrometers
    Micrometer2,
    /// Square Inches
    Inch2,
    /// Square Foot
    Foot2,
    /// Square Yard
    Yard2,

    // Volumes
    /// Cubic Meters
    Meter3,
    /// Cubic Centimeters
    Centimeter3,
    /// Cubic Millimeters
    Millimeter3,
    /// Cubic Micrometers
    Micrometer3,
    /// Cubic Inches
    Inch3,
    /// Cubic Foot
    Foot3,
    /// Cubic Yard
    Yard3,
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
            Self::Meter => write!(f, "m"),
            Self::Centimeter => write!(f, "cm"),
            Self::Millimeter => write!(f, "mm"),
            Self::Micrometer => write!(f, "µm"),
            Self::Inch => write!(f, "in"),
            Self::Foot => write!(f, "ft"),
            Self::Yard => write!(f, "yd"),

            // Angles
            Self::Deg => write!(f, "deg"),
            Self::DegS => write!(f, "°"),
            Self::Grad => write!(f, "grad"),
            Self::Turn => write!(f, "turn"),
            Self::Rad => write!(f, "rad"),

            // Weights
            Self::Gram => write!(f, "g"),
            Self::Kilogram => write!(f, "kg"),
            Self::Pound => write!(f, "lb"),
            Self::Ounce => write!(f, "oz"),

            // Areas
            Self::Meter2 => write!(f, "m³"),
            Self::Centimeter2 => write!(f, "cm²"),
            Self::Millimeter2 => write!(f, "mm²"),
            Self::Micrometer2 => write!(f, "µm²"),
            Self::Inch2 => write!(f, "in²"),
            Self::Foot2 => write!(f, "ft²"),
            Self::Yard2 => write!(f, "yd²"),

            // Volumes
            Self::Meter3 => write!(f, "m³"),
            Self::Centimeter3 => write!(f, "cm³"),
            Self::Millimeter3 => write!(f, "mm³"),
            Self::Micrometer3 => write!(f, "µm³"),
            Self::Inch3 => write!(f, "in³"),
            Self::Foot3 => write!(f, "ft³"),
            Self::Yard3 => write!(f, "yd³"),

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
            "m" => Ok(Self::Meter),
            "cm" => Ok(Self::Centimeter),
            "mm" => Ok(Self::Millimeter),
            "µm" => Ok(Self::Micrometer),
            "in" => Ok(Self::Inch),
            "\"" => Ok(Self::Inch),
            "ft" => Ok(Self::Foot),
            "\'" => Ok(Self::Foot),
            "yd" => Ok(Self::Foot),

            // Angles
            "deg" => Ok(Self::Deg),
            "°" => Ok(Self::DegS),
            "grad" => Ok(Self::Grad),
            "turn" => Ok(Self::Turn),
            "rad" => Ok(Self::Rad),

            // Weights
            "g" => Ok(Self::Gram),
            "kg" => Ok(Self::Kilogram),
            "lb" => Ok(Self::Pound),
            "oz" => Ok(Self::Ounce),

            // Areas
            "m²" | "m2" => Ok(Self::Meter2),
            "cm²" | "cm2" => Ok(Self::Centimeter2),
            "mm²" | "mm2" => Ok(Self::Millimeter2),
            "µm²" | "µm2" => Ok(Self::Micrometer2),
            "in²" | "in2" => Ok(Self::Inch2),
            "ft²" | "ft2" => Ok(Self::Foot2),
            "yd²" | "yd2" => Ok(Self::Yard2),

            // Volumes
            "m³" | "m3" => Ok(Self::Meter3),
            "cm³" | "cm3" => Ok(Self::Centimeter3),
            "mm³" | "mm3" => Ok(Self::Millimeter3),
            "µm³" | "µm3" => Ok(Self::Micrometer3),
            "in³" | "in3" => Ok(Self::Inch3),
            "ft³" | "ft3" => Ok(Self::Foot3),
            "yd³" | "yd3" => Ok(Self::Yard3),
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
            Self::Meter
            | Self::Centimeter
            | Self::Millimeter
            | Self::Micrometer
            | Self::Inch
            | Self::Foot
            | Self::Yard => Type::Length,
            Self::Deg | Self::DegS | Self::Grad | Self::Turn | Self::Rad => Type::Angle,
            Self::Gram | Self::Kilogram | Self::Pound | Self::Ounce => Type::Weight,
            Self::Meter2
            | Self::Centimeter2
            | Self::Millimeter2
            | Self::Micrometer2
            | Self::Inch2
            | Self::Foot2
            | Self::Yard2 => Type::Area,
            Self::Meter3
            | Self::Centimeter3
            | Self::Millimeter3
            | Self::Micrometer3
            | Self::Inch3
            | Self::Foot3
            | Self::Yard3
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
            Self::Meter => x * 1_000.0_f64,
            Self::Centimeter => x * 10.0_f64,
            Self::Millimeter => x,
            Self::Micrometer => x / 1_000.0_f64,
            Self::Inch => x * 25.4_f64,
            Self::Foot => x * 304.8_f64,
            Self::Yard => x * 914.4_f64,

            // Angles
            Self::Deg | Self::DegS => x / 180. * std::f64::consts::PI,
            Self::Grad => x / 200. * std::f64::consts::PI,
            Self::Turn => x * 2.0 * std::f64::consts::PI,
            Self::Rad => x,

            // Weights
            Self::Gram => x,
            Self::Kilogram => x * 1_000.0_f64,
            Self::Pound => x * 453.59237_f64,
            Self::Ounce => x * 28.349_523_125_f64,

            // Areas
            Self::Meter2 => x * 1_000_000.0_f64,
            Self::Centimeter2 => x * 100.0_f64,
            Self::Millimeter2 => x,
            Self::Micrometer2 => x * 0.000_000_1,
            Self::Inch2 => x * 645.16_f64,
            Self::Foot2 => x * 92_903_043.04_f64,
            Self::Yard2 => x * 836_127.36_f64,

            // Volumes
            Self::Meter3 => x * 1_000_000_000.0_f64,
            Self::Centimeter3 => x * 1_000.0_f64,
            Self::Millimeter3 => x,
            Self::Micrometer3 => x * 0.000_000_000_1,
            Self::Inch3 => x * 16_387.06_f64,
            Self::Foot3 => x * 28_316_846.592_f64,
            Self::Yard3 => x * 764_554_857.984_f64,
            Self::Liter => x * 1_000_000.0_f64,
            Self::Centiliter => x * 10_000.0_f64,
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
