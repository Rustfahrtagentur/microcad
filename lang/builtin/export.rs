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

    /// The node does not contain any export attribute.
    #[error("No export attribute found in node. Mark the node with `#[export(\"filename\")`")]
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
    /// Parameters that as exporter specific attributes to a node.
    ///
    /// Let's assume an exporter `foo` has a node parameter `bar = 23` as parameter value list.
    /// The parameter `bar` can be set to `42` with:
    ///
    /// ```ucad
    /// #[export("myfile.foo")]
    /// #[foo(baz = 42)]
    /// circle(42mm);
    /// ```
    fn parameters(&self) -> ParameterValueList {
        ParameterValueList::default()
    }

    /// Export the node if the node is marked for export.
    fn export(&self, node: &ModelNode, filename: &std::path::Path) -> Result<Value, ExportError>;

    /// The expected node output type of this exporter.
    ///
    /// Reimplement this function when your export output format only accepts specific node types.
    fn node_output_type(&self) -> ModelNodeOutputType {
        ModelNodeOutputType::NotDetermined
    }
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

/// Exporter access.
pub trait ExporterAccess {
    /// Get exporter by id.
    fn exporter_by_id(&self, id: &Id) -> Result<Rc<dyn Exporter>, ExportError>;

    /// Get exporter by filename.
    fn exporter_by_filename(
        &self,
        filename: &std::path::Path,
    ) -> Result<Rc<dyn Exporter>, ExportError>;

    /// Find an exporter by filename, or by id.
    fn find_exporter(
        &self,
        filename: &std::path::Path,
        id: &Option<Id>,
    ) -> Result<Rc<dyn Exporter>, ExportError> {
        match id {
            Some(id) => self.exporter_by_id(id),
            None => self.exporter_by_filename(filename),
        }
    }
}

impl ExporterAccess for ExporterRegistry {
    fn exporter_by_id(&self, id: &Id) -> Result<Rc<dyn Exporter>, ExportError> {
        match self.io.by_id(id) {
            Some(exporter) => Ok(exporter),
            None => Err(ExportError::NoExporterWithId(id.clone())),
        }
    }

    fn exporter_by_filename(
        &self,
        filename: &std::path::Path,
    ) -> Result<Rc<dyn Exporter>, ExportError> {
        self.by_filename(filename)
    }
}
