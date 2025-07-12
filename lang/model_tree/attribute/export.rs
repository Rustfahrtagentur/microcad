// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Export attribute.
//!
//!

use crate::{builtin::Exporter, value::Value};

/// Export attribute, e.g. `#[export("output.svg")`.
#[derive(Clone)]
pub struct ExportAttribute {
    /// Filename.
    pub filename: std::path::PathBuf,
    /// Exporter.
    pub exporter: std::rc::Rc<dyn Exporter>,
}

impl ExportAttribute {
    /// Create a new [`ExportAttribute`] with a filename and exporter.
    pub fn new(filename: std::path::PathBuf, exporter: std::rc::Rc<dyn Exporter>) -> Self {
        Self { filename, exporter }
    }
}

impl From<ExportAttribute> for Value {
    fn from(export_attribute: ExportAttribute) -> Self {
        crate::create_tuple_value!(
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
