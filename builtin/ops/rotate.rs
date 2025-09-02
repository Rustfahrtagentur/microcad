// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::{builtin::*, model::*};

/// Builtin definition for a rotation in 2D and 3D.
#[derive(Debug)]
pub struct Rotate;

impl BuiltinWorkbenchDefinition for Rotate {
    fn id() -> &'static str {
        "rotate"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Operation
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|args| {
            Ok(BuiltinWorkpieceOutput::Transform(
                AffineTransform::Rotation(args.get("matrix")),
            ))
        }
    }

    fn parameters() -> ParameterValueList {
        use microcad_lang::ty::*;

        [(
            Identifier::no_ref("matrix"),
            ParameterValue {
                specified_type: Some(Type::Matrix(MatrixType::new(3, 3))),
                ..Default::default()
            },
        )]
        .into_iter()
        .collect()
    }
}
