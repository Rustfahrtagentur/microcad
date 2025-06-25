// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, model_tree::*, parameter, src_ref::*, syntax::*};

/// Builtin definition for a 2D circle
#[derive(Debug)]
pub struct Translate;

impl BuiltinPartDefinition for Translate {
    fn id() -> &'static str {
        "translate"
    }

    fn node(args: &ArgumentMap) -> EvalResult<ModelNode> {
        Ok(ModelNode::new_operation(
            AffineTransform::Translation(Vec3::new(
                args.get_value::<Scalar>(&Identifier::no_ref("x")),
                args.get_value::<Scalar>(&Identifier::no_ref("y")),
                args.get_value::<Scalar>(&Identifier::no_ref("z")),
            )),
            SrcRef(None),
        ))
    }

    fn parameters() -> ParameterValueList {
        vec![
            parameter!(x: Scalar),
            parameter!(y: Scalar),
            parameter!(z: Scalar),
        ]
        .into()
    }
}

impl microcad_core::RenderHash for Translate {
    fn render_hash(&self) -> Option<u64> {
        None
    }
}
