use crate::{parser::*, r#type::*};

macro_rules! declare_units {
    ($( $(#[$m:meta])* $ident:ident = $string:literal -> $ty:ident $(* $factor:expr)? ,)*) => {
        /// The units that can be used after numbers in the language
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub enum Unit {
            $($(#[$m])* $ident,)*
        }

        impl std::fmt::Display for Unit {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$ident => write!(f, $string), )*
                }
            }
        }

        impl std::str::FromStr for Unit {
            type Err = ParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($string => Ok(Self::$ident), )*
                    _ => Err(ParseError::UnknownUnit(s.to_string())),
                }
            }
        }

        impl Unit {
            pub fn ty(self) -> Type {
                match self {
                    $(Self::$ident => Type::$ty, )*
                }
            }

            pub fn normalize(self, x: f64) -> f64 {
                match self {
                    $(Self::$ident => x $(* $factor as f64)?, )*
                }
            }

        }
    };
}

declare_units! {
    /// No unit was given
    None = "" -> Scalar,

    // Percentages
    Percent = "%" -> Scalar * 0.01,

    // Lengths or Coord

    /// Centimeters
    Cm = "cm" -> Length * 10.0,
    /// Millimeters
    Mm = "mm" -> Length,
    /// inches
    In = "in" -> Length * 25.4,
    /// Meters
    M = "m" -> Length * 1000.0,

    // angles

    /// Degree
    Deg = "deg" -> Angle,
    /// Degree
    DegS = "°" -> Angle,
    /// Gradient
    Grad = "grad" -> Angle * 360./180.,
    /// Turns
    Turn = "turn" -> Angle * 360.,
    /// Radians
    Rad = "rad" -> Angle * 360./std::f32::consts::TAU,

    // Weights

    /// Grams
    G = "g" -> Weight,
    /// Kilograms
    Kg = "kg" -> Weight * 1000.0,
    /// Pounds
    Lb = "lb" -> Weight * 453.59237,
}

impl Parse for Unit {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        use std::str::FromStr;
        match Unit::from_str(pair.as_str()) {
            Ok(unit) => Ok(unit),
            Err(_) => Err(ParseError::UnknownUnit(pair.as_str().to_string())),
        }
    }
}
