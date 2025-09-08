// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) export

use microcad_core::{Color, Scalar, theme::Theme};
use microcad_lang::{Id, builtin::*, model::*, parameter, render::RenderError, value::*};

/// SVG Exporter.
pub struct SvgExporter;

/// Settings for this exporter.
pub struct SvgExporterSettings {
    /// Relative padding (e.g. 0.05 = 5% = padding on each side).
    padding_factor: Scalar,
}

impl Default for SvgExporterSettings {
    fn default() -> Self {
        Self {
            padding_factor: 0.05, // 5% padding on each side.
        }
    }
}

impl SvgExporter {
    /// Generate SVG style string from theme.
    pub fn theme_to_svg_style(theme: &Theme) -> String {
        fn fill_stroke_style(
            class_name: &str,
            fill_color: Color,
            stroke_color: Color,
            stroke_width: Scalar,
        ) -> String {
            format!(
                r#" 
        .{class_name} {{
            fill: {fill_color};
            stroke: {stroke_color};
            stroke-width: {stroke_width};
        }}
        "#,
                fill_color = fill_color.to_svg_color(),
                stroke_color = stroke_color.to_svg_color()
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
        ]
        .into_iter()
        .fold(String::new(), |mut style, item| {
            if let Some(stroke) = item.2 {
                style += &fill_stroke_style(item.0, item.1, item.1, stroke);
                style += &stroke_style(item.0, item.1, stroke)
            }
            style += &fill_style(item.0, item.1);
            style
        });

        style += &fill_stroke_style("entity", theme.entity, theme.outline, 0.4);

        style += r#"
            .active { fill-opacity: 1.0; stroke-opacity: 1.0; }
            .inactive { fill-opacity: 0.3; stroke-opacity: 0.3; }
        "#;

        style
    }
}

impl Exporter for SvgExporter {
    fn model_parameters(&self) -> microcad_lang::value::ParameterValueList {
        [
            parameter!(style: String = String::new()),
            parameter!(fill: String = String::new()),
        ]
        .into_iter()
        .collect()
    }

    fn export(&self, model: &Model, filename: &std::path::Path) -> Result<Value, ExportError> {
        use crate::svg::*;
        use microcad_core::FetchBounds2D;
        let settings = SvgExporterSettings::default();
        let bounds = model.fetch_bounds_2d();

        if bounds.is_valid() {
            let content_rect = bounds
                .enlarge(2.0 * settings.padding_factor)
                .rect()
                .expect("Rect");
            log::debug!("Exporting into SVG file {filename:?}");
            let f = std::fs::File::create(filename)?;
            let mut writer = SvgWriter::new_canvas(
                Box::new(std::io::BufWriter::new(f)),
                model.get_size(),
                content_rect,
                None,
            )?;
            writer.style(&SvgExporter::theme_to_svg_style(
                &model.get_theme().unwrap_or_default(),
            ))?;

            model.write_svg(&mut writer, &SvgTagAttributes::default())?;
            Ok(Value::None)
        } else {
            Err(ExportError::RenderError(RenderError::NothingToRender))
        }
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
