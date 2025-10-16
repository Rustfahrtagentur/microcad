// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Export 2D models to Well-Known Text (WKT).

use std::fmt::Write;

use geo::line_string;
use microcad_core::{Geometries2D, Geometry2D, Transformed2D};
use microcad_lang::{
    Id,
    builtin::{ExportError, Exporter, FileIoInterface},
    model::{Model, OutputType},
    value::Value,
};
use wkt::ToWkt;

/// WKT Exporter.
pub struct WktExporter;

trait WriteWkt {
    fn write_wkt(&self, writer: &mut impl Write) -> std::fmt::Result;
}

impl WriteWkt for Geometries2D {
    fn write_wkt(&self, writer: &mut impl Write) -> std::fmt::Result {
        writeln!(writer, "GEOMETRYCOLLECTION(")?;
        self.iter().try_for_each(|geo| geo.write_wkt(writer))?;
        writeln!(writer, ")")
    }
}

impl WriteWkt for Geometry2D {
    fn write_wkt(&self, writer: &mut impl Write) -> std::fmt::Result {
        match &self {
            Geometry2D::LineString(line_string) => {
                writeln!(writer, "{}", line_string.wkt_string())
            }
            Geometry2D::MultiLineString(multi_line_string) => {
                writeln!(writer, "{}", multi_line_string.wkt_string())
            }
            Geometry2D::Polygon(polygon) => {
                writeln!(writer, "{}", polygon.wkt_string())
            }
            Geometry2D::MultiPolygon(multi_polygon) => {
                writeln!(writer, "{}", multi_polygon.wkt_string())
            }
            Geometry2D::Rect(rect) => {
                writeln!(writer, "{}", rect.wkt_string())
            }
            Geometry2D::Line(line) => {
                writeln!(
                    writer,
                    "{}",
                    line_string![line.0.into(), line.1.into()].wkt_string()
                )
            }
            Geometry2D::Collection(collection) => collection.write_wkt(writer),
        }
    }
}

impl WriteWkt for Model {
    fn write_wkt(&self, writer: &mut impl Write) -> std::fmt::Result {
        let self_ = self.borrow();
        let output = self_.output();
        match output {
            microcad_lang::render::RenderOutput::Geometry2D {
                world_matrix,
                geometry,
                ..
            } => {
                let mat = world_matrix.expect("Some matrix");
                match geometry {
                    Some(geometry) => (*geometry.transformed_2d(&mat)).write_wkt(writer),
                    None => self_
                        .children()
                        .try_for_each(|model| model.write_wkt(writer)),
                }
            }
            _ => Ok(()),
        }
    }
}

impl Exporter for WktExporter {
    fn export(&self, model: &Model, filename: &std::path::Path) -> Result<Value, ExportError> {
        use std::io::Write;

        let mut f = std::fs::File::create(filename)?;
        let mut buffer = String::new();
        model.write_wkt(&mut buffer)?;
        f.write_all(buffer.as_bytes())?;
        Ok(Value::None)
    }

    fn output_type(&self) -> OutputType {
        OutputType::Geometry2D
    }
}

impl FileIoInterface for WktExporter {
    fn id(&self) -> Id {
        Id::new("wkt")
    }
}
