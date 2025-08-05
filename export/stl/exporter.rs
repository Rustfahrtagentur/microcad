// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! STL exporter.

use microcad_lang::{
    Id,
    builtin::{ExportError, Exporter, FileIoInterface},
    model::{Model, OutputType},
    value::Value,
};

use crate::stl::{StlWriter, WriteStl};

/// STL Exporter.
pub struct StlExporter;

impl Exporter for StlExporter {
    fn export(&self, model: &Model, filename: &std::path::Path) -> Result<Value, ExportError> {
        let mut f = std::fs::File::create(filename)?;
        let mut writer = StlWriter::new(&mut f)?;
        model.write_stl(&mut writer)?;
        Ok(Value::None)
    }

    fn output_type(&self) -> OutputType {
        OutputType::Geometry3D
    }
}

impl FileIoInterface for StlExporter {
    fn id(&self) -> Id {
        Id::new("stl")
    }
}
