use crate::{tree::NodeInner, Node};

use microcad_core::Identifier;

pub struct ExportSettings {
    pub filename: String,
    pub settings: std::collections::HashMap<Identifier, String>,
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
type ExporterFactory = fn(&str, &ExportSettings) -> Result<Box<dyn Exporter>, crate::Error>;

pub fn export_tree(node: Node, factory: ExporterFactory) -> Result<(), crate::Error> {
    for n in node.descendants() {
        let inner = n.borrow();
        if let NodeInner::Export(ref export_settings) = *inner {
            let filename = export_settings.filename.clone();
            // get file extension
            let ext = std::path::Path::new(&filename)
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap();

            let mut exporter = factory(ext, export_settings)?;

            exporter.export(node.clone())?;
        }
    }

    Ok(())
}
