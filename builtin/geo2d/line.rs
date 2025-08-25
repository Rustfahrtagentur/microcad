// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use geo::coord;
use microcad_core::*;
use microcad_lang::{eval::*, parameter, value::*};

pub struct Line;

impl BuiltinWorkbenchDefinition for Line {
    fn id() -> &'static str {
        "Line"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Primitive2D
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|args| {
            let (x0, y0, x1, y1) = (
                args.get("x0")?,
                args.get("y0")?,
                args.get("x1")?,
                args.get("y1")?,
            );

            Ok(BuiltinWorkpieceOutput::Geometry2D(Geometry2D::Line(
                geo2d::Line(coord! {x: x0, y: y0}.into(), coord! {x: x1, y: y1}.into()),
            )))
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
