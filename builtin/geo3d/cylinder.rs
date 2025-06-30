// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, model_tree::*, parameter, rc::*};

pub struct Cylinder;

impl BuiltinWorkbenchDefinition for Cylinder {
    fn id() -> &'static str {
        "cylinder"
    }

    fn node(args: &ArgumentMap) -> EvalResult<ModelNode> {
        Ok(
            ModelNodeBuilder::new_3d_primitive(Rc::new(Geometry3D::Cylinder(geo3d::Cylinder {
                radius_bottom: args.get("radius_bottom"),
                radius_top: args.get("radius_top"),
                height: args.get("height"),
            })))
            .build(),
        )
    }

    fn parameters() -> ParameterValueList {
        vec![
            parameter!(radius_bottom: Scalar),
            parameter!(radius_top: Scalar),
            parameter!(height: Scalar),
        ]
        .into()
    }
}
