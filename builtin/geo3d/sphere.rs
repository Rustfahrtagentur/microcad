// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, model_tree::*, parameter, rc::*, src_ref::*, syntax::*};

pub struct Sphere;

impl BuiltinPartDefinition for Sphere {
    fn id() -> &'static str {
        "sphere"
    }

    fn node(args: &ArgumentMap) -> EvalResult<ModelNode> {
        Ok(ModelNode::new_element(Refer::none(Element::Primitive3D(
            Rc::new(geo3d::Geometry::Sphere(geo3d::Sphere {
                radius: args.get_value::<Scalar>(&Identifier::no_ref("radius")),
            })),
        ))))
    }

    fn parameters() -> ParameterValueList {
        vec![parameter!(radius: Scalar)].into()
    }
}
