// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) export

use std::io::BufWriter;

use microcad_core::{Color, Scalar, theme::Theme};
use microcad_lang::{Id, builtin::*, model::*, parameter, value::*};

use crate::svg::{SvgTagAttributes, WriteSvg, writer::SvgWriter};

/// SVG Exporter
pub struct SvgExporter;

impl SvgExporter {
    /// Generate SVG style string from theme.
    pub fn theme_to_svg_style(theme: &Theme) -> String {
        fn fill_stroke_style(class_name: &str, color: Color, stroke_width: Scalar) -> String {
            format!(
                r#" 
        .{class_name} {{
            fill: {color};
            stroke: {color};
            stroke-width: {stroke_width};
        }}
        "#,
                color = color.to_svg_color()
            )
        }

        fn fill_style(class_name: &str, fill: Color) -> String {
            format!(
                r#" 
        .{class_name}-fill {{
            fill: {fill};
            stroke: none;
        }}
        "#,
                fill = fill.to_svg_color()
            )
        }

        fn stroke_style(class_name: &str, stroke: Color, stroke_width: Scalar) -> String {
            format!(
                r#" 
        .{class_name}-stroke {{
            fill: none;
            stroke: {stroke};
            stroke-width: {stroke_width};
        }}
        "#,
                stroke = stroke.to_svg_color()
            )
        }

        let mut style = [
            ("background", theme.background, None),
            ("grid", theme.grid, Some(0.2)),
            ("measure", theme.measure, Some(0.2)),
            ("highlight", theme.highlight, Some(0.2)),
            ("entity", theme.entity, Some(0.4)),
        ]
        .into_iter()
        .fold(String::new(), |mut style, item| {
            if let Some(stroke) = item.2 {
                style += &fill_stroke_style(item.0, item.1, stroke);
                style += &stroke_style(item.0, item.1, stroke)
            }
            style += &fill_style(item.0, item.1);
            style
        });

        style += r#"
            .active { fill-opacity: 1.0; stroke-opacity: 1.0; }
            .inactive { fill-opacity: 0.3; stroke-opacity: 0.3; }
        "#;

        style
    }
}

impl Exporter for SvgExporter {
    fn model_parameters(&self) -> microcad_lang::eval::ParameterValueList {
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

        if let Some(content_rect) = model.fetch_bounds_2d().rect() {
            let mut writer = SvgWriter::new_canvas(
                Box::new(BufWriter::new(f)),
                model.get_size(),
                *content_rect,
                None,
            )?;
            writer.style(&SvgExporter::theme_to_svg_style(
                &model.get_theme().unwrap_or_default(),
            ))?;

            model.write_svg(&mut writer, &SvgTagAttributes::default())?;
        }
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
