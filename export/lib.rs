// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Export model nodes to files  

pub mod ply;
pub mod stl;
pub mod svg;

use microcad_lang::{model_tree::*, syntax::*, value::*};
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
    fn id() -> &'static str;

    /// Return file extensions
    fn file_extensions() -> Vec<&'static str> {
        vec![Self::id()]
    }

    fn fetch_attributes_by_id(node: &ModelNode, id: &Identifier) -> Option<Value> {
        let b = node.borrow();
        let attributes = b.attributes();

        attributes.get(id).cloned()
    }

    fn fetch_export_attributes(node: &ModelNode) -> Option<Tuple> {
        if let Some(value) = Self::fetch_attributes_by_id(node, &Identifier::no_ref("export")) {
            match value {
                Value::Tuple(tuple) => Some(*tuple.clone()),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Fetch attributes by exporters `id()`.
    fn fetch_attributes(node: &ModelNode) -> Option<Value> {
        let b = node.borrow();
        let attributes = b.attributes();

        attributes.get(&Identifier::no_ref(Self::id())).cloned()
    }

    /// Export the node if the node is marked for export.
    fn export(&mut self, node: ModelNode) -> Result<Value, ExportError>;
}

pub fn export(node: ModelNode, settings: Tuple) {
    node.borrow_mut().attributes_mut().set_export(settings);
}
