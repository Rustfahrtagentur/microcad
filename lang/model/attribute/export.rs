// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Export attribute.

use crate::{
    builtin::{ExportError, Exporter},
    model::Model,
    value::Value,
};
use cgmath::SquareMatrix;
use microcad_core::{Mat4, RenderResolution, Size2D};

/// Export attribute, e.g. `#[export: "output.svg"]`.
#[derive(Clone)]
pub struct ExportCommand {
    /// Filename.
    pub filename: std::path::PathBuf,
    /// Resolution
    pub resolution: RenderResolution,
    /// Exporter.
    pub exporter: std::rc::Rc<dyn Exporter>,
    /// Layer selector.
    pub layers: Vec<String>,
    /// Size.
    pub size: Size2D,
}

impl ExportCommand {
    /// Export the model. By the settings in the attribute.
    pub fn export(&self, model: &Model) -> Result<Value, ExportError> {
        model.set_matrix(Mat4::identity());
        model.set_resolution(self.resolution.clone());
        model.render();

        self.exporter.export(model, &self.filename)
    }
}

impl From<ExportCommand> for Value {
    fn from(export_attribute: ExportCommand) -> Self {
        crate::create_tuple_value!(
            filename = Value::String(String::from(
                export_attribute.filename.to_str().expect("PathBuf"),
            )),
            id = Value::String(export_attribute.exporter.id().to_string()),
            layers = if export_attribute.layers.is_empty() {
                Value::None
            } else {
                Value::Array(
                    export_attribute
                        .layers
                        .iter()
                        .map(|s| Value::String(s.clone()))
                        .collect(),
                )
            },
            size = Value::from(export_attribute.size)
        )
    }
}

impl std::fmt::Debug for ExportCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Export: {id} => {filename}",
            id = self.exporter.id(),
            filename = self.filename.display()
        )
    }
}

impl std::fmt::Display for ExportCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "\"{filename}\" with exporter `{id}`{layers}",
            filename = self.filename.display(),
            id = self.exporter.id(),
            layers = if self.layers.is_empty() {
                String::from(" (all layers)")
            } else {
                format!(" (layers = [{layers}])", layers = self.layers.join(", "))
            }
        )
    }
}
