// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad color syntax element

use std::str::FromStr;

/// A color with RGBA channels
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    /// red value
    pub r: f32,
    /// green value
    pub g: f32,
    /// blue value
    pub b: f32,
    /// alpha value
    pub a: f32,
}

impl Color {
    /// Create new color.
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Create new color from RGBA values.
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::new(r, g, b, a)
    }

    /// Create new color from RGB values. Alpha is 1.0.
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::rgba(r, g, b, 1.0_f32)
    }

    /// Construct a color from a hex string like `#FFCCAA`.
    pub fn from_hex_str(hex: &str) -> Result<Self, ParseColorError> {
        if !hex.starts_with("#") {
            return Err(ParseColorError::ParseColorFromHex(hex.into()));
        }
        let hex = &hex[1..];

        let hex4bit = |pos| u8::from_str_radix(&hex[pos..pos + 1], 16).map(|v| v as f32 / 15.0);
        let hex8bit = |pos| u8::from_str_radix(&hex[pos..pos + 2], 16).map(|v| v as f32 / 255.0);

        match hex.len() {
            // #RGB or #RGBA single digit hex
            3 | 4 => Ok(Color::rgba(
                hex4bit(0)?,
                hex4bit(1)?,
                hex4bit(2)?,
                if hex.len() == 4 { hex4bit(3)? } else { 1.0 },
            )),
            // #RRGGBB or #RRGGBBAA double digit hex
            6 | 8 => Ok(Color::rgba(
                hex8bit(0)?,
                hex8bit(2)?,
                hex8bit(4)?,
                if hex.len() == 8 { hex8bit(6)? } else { 1.0 },
            )),
            _ => Err(ParseColorError::ParseColorFromHex(hex.into())),
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

impl FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blue" => Ok(Self::rgb(0.0, 0.0, 1.0)),
            "red" => Ok(Self::rgb(1.0, 0.0, 0.0)),
            "green" => Ok(Self::rgb(0.0, 1.0, 0.0)),
            "yellow" => Ok(Self::rgb(1.0, 1.0, 0.0)),
            "cyan" => Ok(Self::rgb(0.0, 1.0, 1.0)),
            "magenta" => Ok(Self::rgb(1.0, 0.0, 1.0)),
            "black" => Ok(Self::rgb(0.0, 0.0, 0.0)),
            "white" => Ok(Self::rgb(1.0, 1.0, 1.0)),
            "gray" => Ok(Self::rgb(0.5, 0.5, 0.5)),
            "orange" => Ok(Self::rgb(1.0, 0.5, 0.0)),
            "purple" => Ok(Self::rgb(0.5, 0.0, 0.5)),
            "pink" => Ok(Self::rgb(1.0, 0.75, 0.8)),
            "brown" => Ok(Self::rgb(0.6, 0.3, 0.1)),
            "lime" => Ok(Self::rgb(0.75, 1.0, 0.0)),
            "teal" => Ok(Self::rgb(0.0, 0.5, 0.5)),
            "navy" => Ok(Self::rgb(0.0, 0.0, 0.5)),
            "transparent" => Ok(Self::rgba(0.0, 0.0, 0.0, 0.0)),
            s => {
                if s.starts_with("#") {
                    Self::from_hex_str(s)
                } else {
                    Err(ParseColorError::UnknownColorName(s.to_string()))
                }
            }
        }
    }
}

use thiserror::Error;

/// An error when parsing a color from a string
#[derive(Error, Debug)]
pub enum ParseColorError {
    /// Unknown color name.
    #[error("Unknown color name: {0}")]
    UnknownColorName(String),

    /// Unknown color name.
    #[error("Could not parse color from hex string: {0}")]
    ParseColorFromHex(String),

    /// Error parsing integer.
    #[error("Error parsing integer literal: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}
