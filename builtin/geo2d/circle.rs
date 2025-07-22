// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, model::*, parameter, rc::*, value::*};

pub struct Circle;

impl BuiltinWorkbenchDefinition for Circle {
    fn id() -> &'static str {
        "circle"
    }

    fn model(args: &Tuple) -> EvalResult<Model> {
        Ok(
            ModelBuilder::new_2d_primitive(Rc::new(Geometry2D::Circle(geo2d::Circle {
                radius: args.get("radius"),
                offset: (args.get("cx"), args.get("cy")).into(),
            })))
            .build(),
        )
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
