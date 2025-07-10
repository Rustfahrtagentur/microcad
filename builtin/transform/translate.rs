// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, model_tree::*, parameter};

/// Builtin definition for a 2D circle
#[derive(Debug)]
pub struct Translate;

impl BuiltinWorkbenchDefinition for Translate {
    fn id() -> &'static str {
        "translate"
    }

    fn node(args: &ArgumentMap) -> EvalResult<ModelNode> {
        Ok(
            ModelNodeBuilder::new_transform(AffineTransform::Translation(Vec3::new(
                args.get("x"),
                args.get("y"),
                args.get("z"),
            )))
            .build(),
        )
    }

    fn parameters() -> ParameterValueList {
        [
            parameter!(x: Scalar),
            parameter!(y: Scalar),
            parameter!(z: Scalar),
        ]
        .into_iter()
        .collect()
    }
}
