// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, model_tree::*, parameter, rc::*, src_ref::*, syntax::*};

pub struct Circle;

impl BuiltinPartDefinition for Circle {
    fn id() -> &'static str {
        "circle"
    }

    fn node(args: &ArgumentMap) -> EvalResult<ModelNode> {
        Ok(ModelNode::new_element(Refer::none(Element::Primitive2D(
            Rc::new(Geometry2D::Circle(geo2d::Circle {
                radius: args.get_value::<Scalar>(&Identifier::no_ref("radius")),
                offset: Vec2::new(0.0, 0.0),
            })),
        ))))
    }

    fn parameters() -> ParameterValueList {
        vec![parameter!(radius: Scalar)].into()
    }
}
