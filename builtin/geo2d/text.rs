// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{builtin::*, render::*};

/// Pie geometry with offset.
#[derive(Debug, Clone)]
pub struct Text {
    /// Text height.
    pub height: Scalar,

    /// Text.
    pub text: String,

    /// Font file (*.ttf or *.otf).
    pub font_file: String,
}

impl From<Text> for BuiltinWorkpieceOutput {
    fn from(text: Text) -> Self {
        BuiltinWorkpieceOutput::Primitive2D(Box::new(text))
    }
}

impl Render<Geometry2D> for Text {
    fn render(&self, resolution: &RenderResolution) -> Geometry2D {
        let font_data = if self.font_file.is_empty() {
            Vec::from(include_bytes!("../assets/dosis-regular.ttf"))
        } else {
            std::fs::read(&self.font_file).expect("Failed to read font file")
        };

        // Load the font into rusttype
        let font = rusttype::Font::try_from_bytes(font_data.as_slice())
            .expect("Failed to load font into rusttype");

        let options = geo_rusttype::TextOptions::new(self.height as f32, font, None, None);
        let polygons = geo_rusttype::text_to_multi_polygon(&self.text, options);

        use geo::Simplify;
        Geometry2D::MultiPolygon(polygons.simplify(resolution.linear))
    }
}

impl RenderWithContext<Geometry2DOutput> for Text {
    fn render_with_context(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, _| Ok(self.render(&context.current_resolution())))
    }
}

impl BuiltinWorkbenchDefinition for Text {
    fn id() -> &'static str {
        "Text"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Primitive2D
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|args| {
            Ok(Text {
                height: args.get("height"),
                text: args.get("text"),
                font_file: args.get("font_file"),
            }
            .into())
        }
    }

    fn parameters() -> ParameterValueList {
        [
            parameter!(height: Scalar),
            parameter!(text: String),
            parameter!(font_file: String = String::new()),
        ]
        .into_iter()
        .collect()
    }
}
