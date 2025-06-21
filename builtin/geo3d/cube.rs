// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, model_tree::*, parameter, rc::*};

pub struct Cube;

impl BuiltinPartDefinition for Cube {
    fn id() -> &'static str {
        "cube"
    }

    fn node(args: &ArgumentMap) -> EvalResult<ModelNode> {
        Ok(
            ModelNodeBuilder::new_3d_primitive(Rc::new(Geometry3D::Cube(geo3d::Cube {
                size: Vec3::new(args.get("size_x"), args.get("size_y"), args.get("size_z")),
            })))
            .build(),
        )
    }

    fn parameters() -> ParameterValueList {
        vec![
            parameter!(size_x: Scalar),
            parameter!(size_y: Scalar),
            parameter!(size_z: Scalar),
        ]
        .into()
    }
}
