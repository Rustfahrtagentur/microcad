// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::builtin::*;

pub struct Pie;

impl BuiltinWorkbenchDefinition for Pie {
    fn id() -> &'static str {
        "Pie"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Primitive2D
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        use microcad_core::*;

        &|args| {
            Ok(BuiltinWorkpieceOutput::Primitive2D(Geometry2D::Circle(
                geo2d::Circle {
                    radius: args.get("radius"),
                    offset: (args.get("cx"), args.get("cy")).into(),
                },
            )))
        }
    }

    fn parameters() -> ParameterValueList {
        [
            parameter!(radius: Scalar),
            parameter!(cx: Scalar = 0.0),
            parameter!(cy: Scalar = 0.0),
            parameter,
        ]
        .into_iter()
        .collect()
    }
}
