// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model node export

use std::rc::Rc;

use crate::{Id, model_tree::*, value::*};

use thiserror::Error;

/// Export error stub.
#[derive(Error, Debug)]
pub enum ExportError {
    /// IO Error.
    #[error("IO Error")]
    IoError(#[from] std::io::Error),

    /// The node does not contain any export metadata.
    #[error("No export metadata found in node. Mark the node with `#[export(\"filename\")`")]
    NoExportAttribute,

    /// Export with id exists already
    #[error("Exporter with id `{0}` exists already!")]
    ExporterWithIdExistsAlready(Id),
}

pub trait Exporter {
    fn id(&self) -> Id;

    /// Return file extensions
    fn file_extensions(&self) -> Vec<Id>;

    /// Export the node if the node is marked for export.
    fn export(&mut self, node: ModelNode) -> Result<Value, ExportError>;
}

#[derive(Default)]
pub struct ExporterRegistry {
    exporters_by_id: std::collections::HashMap<Id, Rc<dyn Exporter>>,
    exporters_by_file_extension: std::collections::HashMap<Id, Vec<Rc<dyn Exporter>>>,
}

impl ExporterRegistry {
    fn add(&mut self, exporter: impl Exporter + 'static) -> Result<(), ExportError> {
        let rc = Rc::new(exporter);
        let id = rc.id();

        if self.exporters_by_id.contains_key(&id) {
            return Err(ExportError::ExporterWithIdExistsAlready(id));
        }

        self.exporters_by_id.insert(id, rc.clone());

        let extensions = rc.file_extensions();
        for ext in extensions {
            if self.exporters_by_file_extension.contains_key(&ext) {
                self.exporters_by_file_extension
                    .get_mut(&ext)
                    .expect("Exporter list")
                    .push(rc.clone());
            } else {
                self.exporters_by_file_extension
                    .insert(ext, vec![rc.clone()]);
            }
        }

        Ok(())
    }
}
