// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Export attribute.

use cgmath::SquareMatrix;
use microcad_core::{Mat4, RenderResolution};

use crate::{
    builtin::{ExportError, Exporter},
    model_tree::ModelNode,
    tuple_value,
    value::Value,
};

/// Export attribute, e.g. `#[export("output.svg")`.
#[derive(Clone)]
pub struct ExportAttribute {
    /// Filename.
    pub filename: std::path::PathBuf,
    /// Resolution
    pub resolution: RenderResolution,
    /// Exporter.
    pub exporter: std::rc::Rc<dyn Exporter>,
}

impl ExportAttribute {
    /// Export the node. By the settings in the attribute.
    pub fn export(&self, node: &ModelNode) -> Result<Value, ExportError> {
        node.set_matrix(Mat4::identity());
        node.set_resolution(self.resolution.clone());

        self.exporter.export(node, &self.filename)
    }
}

impl From<ExportAttribute> for Value {
    fn from(export_attribute: ExportAttribute) -> Self {
        tuple_value!(
            filename = Value::String(String::from(
                export_attribute.filename.to_str().expect("PathBuf"),
            )),
            id = Value::String(export_attribute.exporter.id().to_string())
        )
    }
}

impl std::fmt::Debug for ExportAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Export: {id} => {filename}",
            id = self.exporter.id(),
            filename = self.filename.display()
        )
    }
}

impl std::fmt::Display for ExportAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "\"{filename}\" with exporter `{id}`",
            filename = self.filename.display(),
            id = self.exporter.id()
        )
    }
}
