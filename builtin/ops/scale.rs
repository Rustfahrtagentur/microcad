// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{builtin::*, model::*};

/// Builtin definition for a translation.
#[derive(Debug)]
pub struct Scale;

impl BuiltinWorkbenchDefinition for Scale {
    fn id() -> &'static str {
        "scale"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Transform
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        use microcad_lang::value::*;

        &|args| {
            Ok(BuiltinWorkpieceOutput::Transform(AffineTransform::Scale(
                Vec3::new(args.get("x"), args.get("y"), args.get("z")),
            )))
        }
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
