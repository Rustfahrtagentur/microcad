// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{builtin::*, render::*};

/// Circle with offset.
#[derive(Debug, Clone)]
pub struct Circle(microcad_core::Circle);

impl Render<Geometry2D> for Circle {
    fn render(&self, resolution: &RenderResolution) -> Geometry2D {
        Geometry2D::Polygon(self.0.render(resolution))
    }
}

impl RenderWithContext<Geometry2DOutput> for Circle {
    fn render_with_context(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, _| Ok(self.render(&context.current_resolution())))
    }
}

impl BuiltinWorkbenchDefinition for Circle {
    fn id() -> &'static str {
        "Circle"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Primitive2D
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        use microcad_core::*;

        &|args| {
            Ok(BuiltinWorkpieceOutput::Primitive2D(Box::new(Circle(
                Circle {
                    radius: args.get("radius"),
                    offset: (args.get("cx"), args.get("cy")).into(),
                },
            ))))
        }
    }

    fn parameters() -> ParameterValueList {
        [
            parameter!(radius: Scalar),
            parameter!(cx: Scalar = 0.0),
            parameter!(cy: Scalar = 0.0),
        ]
        .into_iter()
        .collect()
    }
}
