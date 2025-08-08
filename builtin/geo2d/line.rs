// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use geo::coord;
use microcad_core::*;
use microcad_lang::{eval::*, model::*, parameter, rc::*, value::*};

pub struct Line;

impl BuiltinWorkbenchDefinition for Line {
    fn id() -> &'static str {
        "line"
    }

    fn model(args: &Tuple) -> EvalResult<Model> {
        let (x0, y0, x1, y1) = (
            args.get("x0")?,
            args.get("y0")?,
            args.get("x1")?,
            args.get("y1")?,
        );

        Ok(
            ModelBuilder::new_2d_primitive(Rc::new(Geometry2D::Line(geo2d::Line(
                coord! {x: x0, y: y0}.into(),
                coord! {x: x1, y: y1}.into(),
            ))))
            .build(),
        )
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
