// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use geo::coord;
use microcad_core::*;
use microcad_lang::{eval::*, model_tree::*, parameter, rc::*, src_ref::*, syntax::*};

pub struct Rect;

impl BuiltinPartDefinition for Rect {
    fn id() -> &'static str {
        "rect"
    }

    fn node(args: &ArgumentMap) -> EvalResult<ModelNode> {
        let width: Scalar = args.get_value(&Identifier::no_ref("width"));
        let height: Scalar = args.get_value(&Identifier::no_ref("height"));
        let x = args.get_value(&Identifier::no_ref("x"));
        let y = args.get_value(&Identifier::no_ref("y"));

        Ok(ModelNode::new_element(Refer::none(Element::Primitive2D(
            Rc::new(Geometry2D::Rect(geo2d::Rect::new(
                coord! {x: x, y: y},
                coord! {x: x + width, y: y + height},
            ))),
        ))))
    }

    fn parameters() -> ParameterValueList {
        vec![
            parameter!(width: Scalar),
            parameter!(height: Scalar),
            parameter!(x: Scalar),
            parameter!(y: Scalar),
        ]
        .into()
    }
}
