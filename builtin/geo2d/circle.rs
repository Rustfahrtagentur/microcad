// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, model_tree::*, parameter, rc::*};

pub struct Circle;

impl BuiltinPartDefinition for Circle {
    fn id() -> &'static str {
        "circle"
    }

    fn node(args: &ArgumentMap) -> EvalResult<ModelNode> {
        Ok(
            ModelNodeBuilder::new_2d_primitive(Rc::new(Geometry2D::Circle(geo2d::Circle {
                radius: args.get("radius"),
                offset: Vec2::new(args.get("cx"), args.get("cy")),
            })))
            .build(),
        )
    }

    fn parameters() -> ParameterValueList {
        vec![
            parameter!(radius: Scalar),
            parameter!(cx: Scalar = 0.0),
            parameter!(cy: Scalar = 0.0),
        ]
        .into()
    }
}
