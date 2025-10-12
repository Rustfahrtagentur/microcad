// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::builtin::*;

/// Built-in line primitive.
pub struct Line(geo2d::Line);

impl Line {
    pub fn new(x0: Scalar, y0: Scalar, x1: Scalar, y1: Scalar) -> Self {
        use geo::coord;

        Self(geo2d::Line(
            coord! {x: x0, y: y0}.into(),
            coord! {x: x1, y: y1}.into(),
        ))
    }
}

impl Render<Geometry2D> for Line {
    fn render(&self, _: &RenderResolution) -> Geometry2D {
        Geometry2D::Line(self.0.clone())
    }
}

impl BuiltinWorkbenchDefinition for Line {
    fn id() -> &'static str {
        "Line"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Primitive2D
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|args| {
            Ok(BuiltinWorkpieceOutput::Primitive2D(Box::new(Line::new(
                args.get("x0"),
                args.get("y0"),
                args.get("x1"),
                args.get("y1"),
            ))))
        }
    }

    fn parameters() -> ParameterValueList {
        [
            parameter!(x0: Scalar),
            parameter!(y0: Scalar),
            parameter!(x1: Scalar),
            parameter!(y1: Scalar),
        ]
        .into_iter()
        .collect()
    }
}
