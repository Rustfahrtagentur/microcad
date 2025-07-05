// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, model_tree::*, rc::*, *};

pub struct Sphere;

impl BuiltinWorkbenchDefinition for Sphere {
    fn id() -> &'static str {
        "sphere"
    }

    fn node(args: &ArgumentMap) -> EvalResult<ModelNode> {
        Ok(
            ModelNodeBuilder::new_3d_primitive(Rc::new(geo3d::Geometry::Sphere(geo3d::Sphere {
                radius: args.get(&id!("radius")),
            })))
            .build(),
        )
    }

    fn parameters() -> ParameterValueList {
        [parameter!(radius: Scalar)].into_iter().collect()
    }
}
