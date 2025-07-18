// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad style element

use std::str::FromStr;

use crate::{Color, Scalar};

/// A style element.
#[derive(Debug, Clone, Default)]
pub struct Style {
    /// Fill color.
    pub fill: Option<Color>,
    /// Stroke color.
    pub stroke: Option<Color>,
    /// Stroke width.
    pub stroke_width: Option<Scalar>,
    /// Font size.
    pub font_size: Option<Scalar>,
}

impl Style {
    /// Function to convert the Style struct into an SVG style tag attribute string.
    pub fn to_svg_style(&self) -> String {
        [
            self.fill
                .as_ref()
                .map(|fill| format!("fill:{}", fill.to_svg_color())),
            self.stroke
                .as_ref()
                .map(|stroke| format!("stroke:{}", stroke.to_svg_color())),
            self.stroke_width
                .map(|stroke_width| format!("stroke-width:{stroke_width}")),
            self.font_size
                .map(|font_size| format!("font-size:{font_size}mm")),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(";")
    }
}

/// Color theme.
pub struct Theme {
    /// Font family.
    pub font_family: Option<String>,

    /// Default style.
    pub default: Style,

    /// Style for each layer.
    pub layers: std::collections::HashMap<String, Style>,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            font_family: Some("Arial".to_string()),
            default: Style {
                fill: Some(Color::new(1.0, 1.0, 1.0, 1.0)),   // White
                stroke: Some(Color::new(0.0, 0.0, 0.0, 1.0)), // Black
                stroke_width: Some(1.0),
                font_size: Some(12.0),
            },
            layers: std::collections::HashMap::from([
                (
                    "result".into(),
                    Style {
                        fill: None,
                        stroke: Some(Color::from_str("#111111").expect("")),
                        stroke_width: Some(1.0),
                        ..Default::default()
                    },
                ),
                (
                    "operands".into(),
                    Style {
                        fill: None,
                        stroke: Some(Color::from_str("#555555").expect("")),
                        stroke_width: Some(1.0),
                        ..Default::default()
                    },
                ),
                (
                    "measures".into(),
                    Style {
                        fill: None,
                        stroke: Some(Color::from_str("#555555").expect("")),
                        stroke_width: Some(1.0),
                        ..Default::default()
                    },
                ),
            ]),
        }
    }
}
