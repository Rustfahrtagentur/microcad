// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µCAD unit parser entity

use crate::{errors::*, parser::*, r#type::*};

/// The units that can be used after numbers in the language"
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Unit {
    /// No unit was given
    None,
    /// Percents
    Percent,
    /// Centimeters
    Cm,
    /// Millimeters
    Mm,
    /// Inches
    In,
    /// Meters
    M,
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
}
impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::None => write!(f, ""),
            Self::Percent => write!(f, "%"),
            Self::Cm => write!(f, "cm"),
            Self::Mm => write!(f, "mm"),
            Self::In => write!(f, "in"),
            Self::M => write!(f, "m"),
            Self::Deg => write!(f, "deg"),
            Self::DegS => write!(f, "°"),
            Self::Grad => write!(f, "grad"),
            Self::Turn => write!(f, "turn"),
            Self::Rad => write!(f, "rad"),
            Self::G => write!(f, "g"),
            Self::Kg => write!(f, "kg"),
            Self::Lb => write!(f, "lb"),
            Self::Cm2 => write!(f, "cm²"),
            Self::Mm2 => write!(f, "mm²"),
            Self::In2 => write!(f, "in²"),
            Self::M2 => write!(f, "m³"),
            Self::Cm3 => write!(f, "cm³"),
            Self::Mm3 => write!(f, "mm³"),
            Self::In3 => write!(f, "in³"),
            Self::M3 => write!(f, "m³"),
        }
    }
}
impl std::str::FromStr for Unit {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::None),
            "%" => Ok(Self::Percent),
            "cm" => Ok(Self::Cm),
            "mm" => Ok(Self::Mm),
            "in" => Ok(Self::In),
            "m" => Ok(Self::M),
            "deg" => Ok(Self::Deg),
            "°" => Ok(Self::DegS),
            "grad" => Ok(Self::Grad),
            "turn" => Ok(Self::Turn),
            "rad" => Ok(Self::Rad),
            "g" => Ok(Self::G),
            "kg" => Ok(Self::Kg),
            "lb" => Ok(Self::Lb),
            "cm²" => Ok(Self::Cm2),
            "mm²" => Ok(Self::Mm2),
            "in²" => Ok(Self::In2),
            "m²" => Ok(Self::M2),
            "cm³" => Ok(Self::Cm3),
            "mm³" => Ok(Self::Mm3),
            "in³" => Ok(Self::In3),
            "m³" => Ok(Self::M3),
            _ => Err(ParseError::UnknownUnit(s.to_string())),
        }
    }
}
impl Unit {
    /// Return type to use with this unit
    pub fn ty(self) -> Type {
        match self {
            Self::None | Self::Percent => Type::Scalar,
            Self::Cm | Self::Mm | Self::In | Self::M => Type::Length,
            Self::Deg | Self::DegS | Self::Grad | Self::Turn | Self::Rad => Type::Angle,
            Self::G | Self::Kg | Self::Lb => Type::Weight,
            Self::Cm2 | Self::Mm2 | Self::In2 | Self::M2 => Type::Area,
            Self::Cm3 | Self::Mm3 | Self::In3 | Self::M3 => Type::Volume,
        }
    }
    /// Normalize value to mm, deg or gram
    pub fn normalize(self, x: f64) -> f64 {
        match self {
            Self::None => x,
            Self::Percent => x * 0.01_f64,
            Self::Cm => x * 10.0_f64,
            Self::Mm => x,
            Self::In => x * 25.4_f64,
            Self::M => x * 1000.0_f64,
            Self::Deg => x,
            Self::DegS => x,
            Self::Grad => x * (360. / 180.),
            Self::Turn => x * 360_f64,
            Self::Rad => x * (360. / std::f32::consts::TAU) as f64,
            Self::G => x,
            Self::Kg => x * 1000.0_f64,
            Self::Lb => x * 453.59237_f64,
            Self::Cm2 => x * 100.0_f64,
            Self::Mm2 => x,
            Self::In2 => x * 645.16_f64,
            Self::M2 => x * 1000000.0_f64,
            Self::Cm3 => x * 1000.0_f64,
            Self::Mm3 => x,
            Self::In3 => x * 16387.06_f64,
            Self::M3 => x * 1000000000.0_f64,
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
