// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Export attribute.

use crate::{
    builtin::{ExportError, Exporter},
    model::Model,
    render::RenderContext,
    value::Value,
};
use microcad_core::RenderResolution;

/// Export attribute, e.g. `#[export: "output.svg"]`.
#[derive(Clone)]
pub struct ExportCommand {
    /// Filename.
    pub filename: std::path::PathBuf,
    /// Resolution
    pub resolution: RenderResolution,
    /// Exporter.
    pub exporter: std::rc::Rc<dyn Exporter>,
}

impl ExportCommand {
    /// Export the model. By the settings in the attribute.
    pub fn export(&self, model: &Model) -> Result<Value, ExportError> {
        let mut render_context = RenderContext::init(model, RenderResolution::default())?;

        use crate::render::Render;
        self.exporter
            .export(&model.render(&mut render_context)?, &self.filename)
    }
}

impl From<ExportCommand> for Value {
    fn from(export_attribute: ExportCommand) -> Self {
        crate::create_tuple_value!(
            filename = Value::String(String::from(
                export_attribute.filename.to_str().expect("PathBuf"),
            )),
            id = Value::String(export_attribute.exporter.id().to_string())
        )
    }
}

impl std::fmt::Debug for ExportCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Export: {id:?} => {filename}",
            id = self.exporter.id(),
            filename = self.filename.display()
        )
    }
}

impl std::fmt::Display for ExportCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\"{filename}\" with exporter `{id}`",
            filename = self.filename.display(),
            id = self.exporter.id(),
        )
    }
}
