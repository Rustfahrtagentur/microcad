// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model node export

use std::rc::Rc;

use crate::{Id, builtin::file_io::*, eval::*, model_tree::*, value::*};

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

    /// No exporter found for file.
    #[error("No exporter found for file `{0}`")]
    NoExporterForFile(std::path::PathBuf),

    /// No exporter for id.
    #[error("No exporter found with id `{0}`")]
    NoExporterWithId(Id),

    /// No exporter id.
    #[error("Multiple exporters for file extension: {0:?}")]
    MultipleExportersForFileExtension(Vec<Id>),
}

/// Exporter trait.
///
/// Implement this trait for your custom file exporter.
pub trait Exporter: FileIoInterface {
    /// Export the node if the node is marked for export.
    fn export(&mut self, node: ModelNode, args: &ArgumentMap) -> Result<Value, ExportError>;
}

/// Exporter registry.
///
/// A database in which all exporters are stored.
///
/// The registry is used to find exporters by their id and their file extension.
#[derive(Default)]
pub struct ExporterRegistry {
    io: FileIoRegistry<Rc<dyn Exporter>>,
}

impl ExporterRegistry {
    /// Create new registry.
    pub fn new() -> Self {
        Self {
            io: FileIoRegistry::default(),
        }
    }

    /// Add new exporter to the registry.
    ///
    /// TODO Error handling.
    pub fn insert(mut self, exporter: impl Exporter + 'static) -> Self {
        let rc = Rc::new(exporter);
        self.io.insert(rc);
        self
    }

    /// Get exporter by id.
    pub fn by_id(&self, id: &Id) -> Result<Rc<dyn Exporter>, ExportError> {
        self.io
            .by_id(id)
            .ok_or(ExportError::NoExporterWithId(id.clone()))
    }

    /// Get exporter by filename.
    pub fn by_filename(
        &self,
        filename: impl AsRef<std::path::Path>,
    ) -> Result<Rc<dyn Exporter>, ExportError> {
        let importers = self.io.by_filename(filename.as_ref());
        match importers.len() {
            0 => Err(ExportError::NoExporterForFile(std::path::PathBuf::from(
                filename.as_ref(),
            ))),
            1 => Ok(importers.first().expect("One importer").clone()),
            _ => Err(ExportError::MultipleExportersForFileExtension(
                importers.iter().map(|importer| importer.id()).collect(),
            )),
        }
    }
}
