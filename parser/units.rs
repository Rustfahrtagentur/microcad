use crate::langtype::Type;
use crate::parser::{Pair, Parse, ParseError};

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
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($string => Ok(Self::$ident), )*
                    _ => Err(())
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
    /// Gradians
    Grad = "grad" -> Angle * 360./180.,
    /// Turns
    Turn = "turn" -> Angle * 360.,
    /// Radians
    Rad = "rad" -> Angle * 360./std::f32::consts::TAU,
}

impl Parse for Unit {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        use std::str::FromStr;
        Unit::from_str(pair.as_str())
            .map_err(|_| ParseError::UnknownUnit(pair.as_span().as_str().to_string()))
    }
}
