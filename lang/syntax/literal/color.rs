// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad color syntax element

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
    /// Create new color
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}
