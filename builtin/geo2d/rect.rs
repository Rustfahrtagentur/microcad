// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::builtin::*;

pub struct Rect(geo2d::Rect);

impl Rect {
    pub fn new(x: Scalar, y: Scalar, width: Scalar, height: Scalar) -> Self {
        use geo::coord;
        Self(geo2d::Rect::new(
            coord! {x: x, y: y},
            coord! {x: x + width, y: y + height},
        ))
    }
}

impl Render<Geometry2D> for Rect {
    fn render(&self, _: &RenderResolution) -> Geometry2D {
        Geometry2D::Rect(self.0)
    }
}

impl BuiltinWorkbenchDefinition for Rect {
    fn id() -> &'static str {
        "Rect"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Primitive2D
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|args| {
            Ok(BuiltinWorkpieceOutput::Primitive2D(Box::new(Rect::new(
                args.get("x"),
                args.get("y"),
                args.get("width"),
                args.get("height"),
            ))))
        }
    }

    fn parameters() -> ParameterValueList {
        use microcad_lang::parameter;
        [
            parameter!(width: Scalar),
            parameter!(height: Scalar),
            parameter!(x: Scalar),
            parameter!(y: Scalar),
        ]
        .into_iter()
        .collect()
    }
}
