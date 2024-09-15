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
    /// inches
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
}
impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
