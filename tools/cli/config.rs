// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI config.

use serde::Deserialize;

/// Microcad CLI config.
#[derive(Deserialize, Default)]
pub struct Config {
    /// Export settings.
    pub export: Export,
}

impl Config {
    /// Load config from TOML file.
    pub fn load(filename: &std::path::Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(filename)?;

        Ok(toml::from_str(&content)?)
    }
}

/// Export settings.
#[derive(Deserialize)]
pub struct Export {
    /// Default sketch exporter.
    pub sketch: String,
    /// Default part exporter.
    pub part: String,
}

impl Default for Export {
    fn default() -> Self {
        Self {
            sketch: "svg".into(),
            part: "stl".into(),
        }
    }
}
