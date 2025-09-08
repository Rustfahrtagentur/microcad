// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::builtin::*;

pub struct Rect;

impl BuiltinWorkbenchDefinition for Rect {
    fn id() -> &'static str {
        "Rect"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Primitive2D
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        use geo::coord;
        use microcad_core::*;

        &|args| {
            let width: Scalar = args.get("width");
            let height: Scalar = args.get("height");
            let x = args.get("x");
            let y = args.get("y");

            Ok(BuiltinWorkpieceOutput::Primitive2D(Geometry2D::Rect(
                geo2d::Rect::new(coord! {x: x, y: y}, coord! {x: x + width, y: y + height}),
            )))
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
