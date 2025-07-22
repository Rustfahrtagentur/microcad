// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) export

use std::io::BufWriter;

use microcad_lang::{Id, builtin::*, model::*, parameter, value::*};

use crate::svg::{SvgTagAttributes, WriteSvg, writer::SvgWriter};

/// SVG Exporter
pub struct SvgExporter;

impl Exporter for SvgExporter {
    fn parameters(&self) -> microcad_lang::eval::ParameterValueList {
        [
            parameter!(style: String = String::new()),
            parameter!(fill: String = String::new()),
        ]
        .into_iter()
        .collect()
    }

    fn export(&self, model: &Model, filename: &std::path::Path) -> Result<Value, ExportError> {
        let f = std::fs::File::create(filename)?;
        use microcad_core::FetchBounds2D;
        let bounds = model.fetch_bounds_2d();
        let mut writer = SvgWriter::new_canvas(Box::new(BufWriter::new(f)), bounds, None)?;

        model.write_svg(&mut writer, &SvgTagAttributes::default())?;
        Ok(Value::None)
    }

    fn output_type(&self) -> OutputType {
        OutputType::Geometry2D
    }
}

impl FileIoInterface for SvgExporter {
    fn id(&self) -> Id {
        Id::new("svg")
    }
}
