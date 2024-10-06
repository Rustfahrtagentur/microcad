// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µCAD export

use crate::{render::Node, render::NodeInner};

/// Export settings
#[derive(Debug, Default)]
pub struct ExportSettings(toml::Table);

impl std::ops::Deref for ExportSettings {
    type Target = toml::Table;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ExportSettings {
    /// Create export settings with an initial file name
    pub fn with_filename(filename: String) -> Self {
        let mut settings = ExportSettings::default();
        settings
            .0
            .insert("filename".to_string(), toml::Value::String(filename));
        settings
    }

    /// return file name
    pub fn filename(&self) -> Option<String> {
        self.0
            .get("filename")
            .map(|filename| filename.as_str().unwrap().to_string())
    }

    /// Return render precision
    pub fn render_precision(&self) -> f64 {
        self.0
            .get("render_precision")
            .map(|precision| precision.as_float().unwrap())
            .unwrap_or(0.1)
    }

    /// Get exporter ID
    pub fn exporter_id(&self) -> Option<String> {
        if let Some(exporter) = self.0.get("exporter") {
            Some(exporter.to_string())
        } else if let Some(filename) = self.filename() {
            let ext = std::path::Path::new(&filename)
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap();
            Some(ext.to_string())
        } else {
            None
        }
    }
}

/// Exporter trait
pub trait Exporter {
    /// Create from settings
    fn from_settings(settings: &ExportSettings) -> Result<Self, crate::CoreError>
    where
        Self: Sized;

    /// Return file extensions
    fn file_extensions(&self) -> Vec<&str>;

    /// Do export
    fn export(&mut self, node: Node) -> Result<(), crate::CoreError>;
}

/// Short cut to create an export node
pub fn export(export_settings: ExportSettings) -> Node {
    Node::new(NodeInner::Export(export_settings))
}

/// The `ExporterFactory` creates a new exporter based on the file extension and the export settings
type ExporterFactory = fn(&ExportSettings) -> Result<Box<dyn Exporter>, crate::CoreError>;

/// Iterate over all descendent nodes and export the ones with an Export tag
pub fn export_tree(node: Node, factory: ExporterFactory) -> Result<(), crate::CoreError> {
    node.descendants().try_for_each(|n| {
        let inner = n.borrow();
        if let NodeInner::Export(ref export_settings) = *inner {
            factory(export_settings)?.export(n.clone())
        } else {
            Ok(())
        }
    })
}
