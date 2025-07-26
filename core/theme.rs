// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Color theme

use crate::color::Color;

/// Represents a color theme.
#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    /// Name of the theme.
    pub name: String,
    /// Filename of the theme, if it was loaded from file.
    pub filename: Option<String>,
    /// Background color of the drawing canvas
    pub background: Color,
    /// Color used for grid lines
    pub grid: Color,
    /// Color used for selected entities
    pub selection: Color,
    /// Color used for highlighting hovered entities
    pub highlight: Color,
    /// Default color for entities
    pub entity: Color,
    /// Color used for construction lines
    pub construction: Color,
    /// Color for dimensions and annotations
    pub annotation: Color,
    /// Color for snapping indicators
    pub snap_indicator: Color,
    /// Color for guidelines (e.g. inference lines)
    pub guide: Color,
}

impl Theme {
    /// Dark theme.
    pub fn dark() -> Self {
        Self {
            name: "default/dark".into(),
            filename: None,
            background: Color::rgb(0.1, 0.1, 0.1),
            grid: Color::rgb(0.2, 0.2, 0.2),
            selection: Color::rgb(1.0, 0.6, 0.0),
            highlight: Color::rgb(1.0, 1.0, 0.0),
            entity: Color::rgb(0.9, 0.9, 0.9),
            construction: Color::rgb(0.4, 0.4, 0.8),
            annotation: Color::rgb(0.8, 0.8, 0.3),
            snap_indicator: Color::rgb(0.0, 1.0, 1.0),
            guide: Color::rgb(0.6, 0.6, 0.6),
        }
    }

    /// Light theme.
    pub fn light() -> Self {
        Self {
            name: "default/light".into(),
            filename: None,
            background: Color::rgb(1.0, 1.0, 1.0),
            grid: Color::rgb(0.85, 0.85, 0.85),
            selection: Color::rgb(0.0, 0.4, 0.8),
            highlight: Color::rgb(1.0, 0.6, 0.0),
            entity: Color::rgb(0.1, 0.1, 0.1),
            construction: Color::rgb(0.4, 0.4, 0.8),
            annotation: Color::rgb(0.5, 0.5, 0.0),
            snap_indicator: Color::rgb(0.0, 0.8, 0.8),
            guide: Color::rgb(0.6, 0.6, 0.6),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}
