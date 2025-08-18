// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::Vec3;
use microcad_lang::{eval::*, model::*, parameter, syntax::WorkbenchKind, value::*};

/// Builtin definition to orient an object towards an axis.
#[derive(Debug)]
pub struct Orient;

impl BuiltinWorkbenchDefinition for Orient {
    fn id() -> &'static str {
        "orient"
    }

    fn kind() -> WorkbenchKind {
        WorkbenchKind::Operation
    }

    fn model(args: &Tuple) -> EvalResult<Model> {
        Ok(
            ModelBuilder::new_transform(AffineTransform::Rotation(crate::math::orient_z_to(
                Vec3::new(args.get("x")?, args.get("y")?, args.get("z")?),
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
