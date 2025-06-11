// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, model_tree::*, src_ref::*, syntax::*, ty::*};

/// Builtin definition for a 2D circle
#[derive(Debug)]
pub struct Rotate;

impl BuiltinPartDefinition for Rotate {
    fn id() -> &'static str {
        "rotate"
    }

    fn node(args: &ArgumentMap) -> EvalResult<ModelNode> {
        Ok(ModelNode::new_operation(
            AffineTransform::RotationAroundAxis(
                cgmath::Rad(args.get_value::<Scalar>(&Identifier::no_ref("angle"))),
                Vec3::new(
                    args.get_value::<Scalar>(&Identifier::no_ref("x")),
                    args.get_value::<Scalar>(&Identifier::no_ref("y")),
                    args.get_value::<Scalar>(&Identifier::no_ref("z")),
                ),
            ),
            SrcRef(None),
        ))
    }

    fn parameters() -> ParameterList {
        ParameterList::new(
            vec![
                Parameter::no_ref("angle", Type::Angle),
                Parameter::no_ref("x", Type::Scalar),
                Parameter::no_ref("y", Type::Scalar),
                Parameter::no_ref("z", Type::Scalar),
            ]
            .into(),
        )
    }
}

impl microcad_core::RenderHash for Rotate {
    fn render_hash(&self) -> Option<u64> {
        None
    }
}
