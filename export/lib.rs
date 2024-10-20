// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Export of meshes generated by µCAD  

#![warn(missing_docs)]

pub mod ply;
pub mod stl;
pub mod svg;
pub mod tree_dump;

pub use microcad_core::{ExportSettings, CoreError};
use microcad_lang::objecttree::*;

/// Exporter trait
pub trait Exporter {
    /// Create from settings
    fn from_settings(settings: &ExportSettings) -> Result<Self, CoreError>
    where
        Self: Sized;

    /// Return file extensions
    fn file_extensions(&self) -> Vec<&str>;

    /// Do export
    fn export(&mut self, node: ObjectNode) -> Result<(), CoreError>;
}

/// Short cut to create an export node
pub fn export(export_settings: ExportSettings) -> ObjectNode {
    ObjectNode::new(ObjectNodeInner::Export(export_settings))
}

/// The `ExporterFactory` creates a new exporter based on the file extension and the export settings
type ExporterFactory = fn(&ExportSettings) -> Result<Box<dyn Exporter>, CoreError>;

/// Iterate over all descendent nodes and export the ones with an Export tag
pub fn export_tree(node: ObjectNode, factory: ExporterFactory) -> Result<(), CoreError> {
    node.descendants().try_for_each(|n| {
        let inner = n.borrow();
        if let ObjectNodeInner::Export(ref export_settings) = *inner {
            factory(export_settings)?.export(n.clone())
        } else {
            Ok(())
        }
    })
}
