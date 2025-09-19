// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::CoreError;

use std::str::FromStr;

/// Direction
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    /// X direction
    X,
    /// Y direction
    Y,
    /// Z direction
    Z,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::X => write!(f, "X"),
            Direction::Y => write!(f, "Y"),
            Direction::Z => write!(f, "Z"),
        }
    }
}

impl FromStr for Direction {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::X),
            "Y" => Ok(Self::Y),
            "Z" => Ok(Self::Z),
            _ => Err(CoreError::BadAlignment(s.to_string())),
        }
    }
}
