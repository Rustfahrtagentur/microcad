// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, model_tree::*, parameter, rc::*, src_ref::*, syntax::*};

pub struct Cube;

impl BuiltinPartDefinition for Cube {
    fn id() -> &'static str {
        "cube"
    }

    fn node(args: &ArgumentMap) -> EvalResult<ModelNode> {
        Ok(ModelNode::new_element(Refer::none(Element::Primitive3D(
            Rc::new(Geometry3D::Cube(geo3d::Cube {
                size: Vec3::new(
                    args.get_value::<Scalar>(&Identifier::no_ref("size_x")),
                    args.get_value::<Scalar>(&Identifier::no_ref("size_y")),
                    args.get_value::<Scalar>(&Identifier::no_ref("size_z")),
                ),
            })),
        ))))
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
