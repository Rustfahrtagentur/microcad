// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, model_tree::*, parameter, rc::*, value::*};

pub struct Cube;

impl BuiltinWorkbenchDefinition for Cube {
    fn id() -> &'static str {
        "cube"
    }

    fn model(args: &Tuple) -> EvalResult<Model> {
        Ok(
            ModelBuilder::new_3d_primitive(Rc::new(Geometry3D::Cube(geo3d::Cube {
                size: Vec3::new(args.get("size_x"), args.get("size_y"), args.get("size_z")),
            })))
            .build(),
        )
    }

    fn parameters() -> ParameterValueList {
        [
            parameter!(size_x: Scalar),
            parameter!(size_y: Scalar),
            parameter!(size_z: Scalar),
        ]
        .into_iter()
        .collect()
    }
}
