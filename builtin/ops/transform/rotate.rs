// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, model_tree::*, parameter, value::*};

/// Builtin definition for a 2D circle
#[derive(Debug)]
pub struct Rotate;

impl BuiltinWorkbenchDefinition for Rotate {
    fn id() -> &'static str {
        "rotate"
    }

    fn model(args: &Tuple) -> EvalResult<Model> {
        Ok(
            ModelBuilder::new_transform(AffineTransform::RotationAroundAxis(
                cgmath::Rad(args.get("angle")),
                Vec3::new(args.get("x"), args.get("y"), args.get("z")),
            ))
            .build(),
        )
    }

    fn parameters() -> ParameterValueList {
        [
            parameter!(angle: Scalar),
            parameter!(x: Scalar),
            parameter!(y: Scalar),
            parameter!(z: Scalar),
        ]
        .into_iter()
        .collect()
    }
}
