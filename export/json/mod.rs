// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! STL exporter.

use microcad_lang::{
    builtin::{ExportError, Exporter, FileIoInterface},
    model::{Model, OutputType},
    value::Value,
    Id,
};

/// STL Exporter.
pub struct JsonExporter;

impl Exporter for JsonExporter {
    fn export(&self, model: &Model, filename: &std::path::Path) -> Result<Value, ExportError> {
        log::debug!("Exporting model into {filename:?}");
        let mut f = std::fs::File::create(filename)?;
        log::trace!("Model to export:\n{model}");
        let _writer = std::io::BufWriter::new(&mut f);
        todo!("export json");
    }

    fn output_type(&self) -> OutputType {
        OutputType::Geometry3D
    }
}

impl FileIoInterface for JsonExporter {
    fn id(&self) -> Id {
        Id::new("json")
    }
}
