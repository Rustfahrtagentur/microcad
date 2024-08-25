use std::ops::Deref;

use crate::{render::Node, render::NodeInner};

#[derive(Debug, Default)]
pub struct ExportSettings(toml::Table);

impl Deref for ExportSettings {
    type Target = toml::Table;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ExportSettings {
    pub fn with_filename(filename: String) -> Self {
        let mut settings = ExportSettings::default();
        settings
            .0
            .insert("filename".to_string(), toml::Value::String(filename));
        settings
    }

    pub fn filename(&self) -> Option<String> {
        self.0
            .get("filename")
            .map(|filename| filename.as_str().unwrap().to_string())
    }

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

pub trait Exporter {
    fn from_settings(settings: &ExportSettings) -> Result<Self, crate::Error>
    where
        Self: Sized;

    fn file_extensions(&self) -> Vec<&str>;

    fn export(&mut self, node: Node) -> Result<(), crate::Error>;
}

pub fn export(export_settings: ExportSettings) -> Node {
    Node::new(NodeInner::Export(export_settings))
}

/// The `ExporterFactory` creates a new exporter based on the file extension and the export settings
type ExporterFactory = fn(&ExportSettings) -> Result<Box<dyn Exporter>, crate::Error>;

pub fn export_tree(node: Node, factory: ExporterFactory) -> Result<(), crate::Error> {
    for n in node.descendants() {
        let inner = n.borrow();
        if let NodeInner::Export(ref export_settings) = *inner {
            let mut exporter = factory(export_settings)?;
            exporter.export(node.clone())?;
        }
    }

    Ok(())
}
