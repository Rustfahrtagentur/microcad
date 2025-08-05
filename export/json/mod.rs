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
        let mut f = std::fs::File::create(filename)?;
        let writer = std::io::BufWriter::new(&mut f);
        match serde_json::to_writer_pretty(writer, model) {
            Ok(_) => Ok(Value::None),
            Err(err) => todo!("{err}"),
        }
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
