// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Export model nodes to files  

pub mod ply;
pub mod stl;
pub mod svg;

use microcad_lang::{model_tree::ModelNode, value::Tuple};
use thiserror::Error;

/// Export error stub.
#[derive(Error, Debug)]
pub enum ExportError {
    /// IO Error.
    #[error("IO Error")]
    IoError(#[from] std::io::Error),

    /// The node does not contain any export metadata.
    #[error("No export metadata found in node. Mark the node with `#[export(\"filename\")`")]
    NoExportMetadata,
}

/// Exporter trait
pub trait Exporter {
    /// Return file extensions
    fn file_extensions(&self) -> Vec<&str>;

    /// Fetch export metadata from a single node.
    fn fetch_export_metadata(&self, node: &ModelNode) -> Option<Tuple>;

    /// Export the node if the node is marked for export.
    fn export(&mut self, node: ModelNode) -> Result<(), ExportError>;
}
