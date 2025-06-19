// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, model_tree::*, parameter, rc::*, src_ref::*, syntax::*};

pub struct Cylinder;

impl BuiltinPartDefinition for Cylinder {
    fn id() -> &'static str {
        "cylinder"
    }

    fn node(args: &ArgumentMap) -> EvalResult<ModelNode> {
        Ok(ModelNode::new_element(Refer::none(Element::Primitive3D(
            Rc::new(Geometry3D::Cylinder(geo3d::Cylinder {
                radius_bottom: args.get_value::<Scalar>(&Identifier::no_ref("radius_bottom")),
                radius_top: args.get_value::<Scalar>(&Identifier::no_ref("radius_top")),
                height: args.get_value::<Scalar>(&Identifier::no_ref("height")),
            })),
        ))))
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
