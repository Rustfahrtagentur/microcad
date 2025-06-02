// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Color built-ins.

use std::str::FromStr;

use crate::{parse::ParseError, syntax::Color};

impl Color {
    /// Construct a color from a hex string like `#FFCCAA`.
    pub fn from_hex_str(hex: &str) -> Result<Self, ParseError> {
        if !hex.starts_with("#") {
            return Err(ParseError::ParseColorFromHex(hex.to_string()));
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
            _ => Err(ParseError::ParseColorFromHex(hex.into())),
        }
    }
}

impl FromStr for Color {
    type Err = ParseError;

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
                    Err(ParseError::UnknownColorName(s.to_string()))
                }
            }
        }
    }
}
