// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::{
    eval::*,
    model::*,
    syntax::{Identifier, WorkbenchKind},
    ty::{MatrixType, Type},
    value::*,
};

/// Builtin definition for a rotation in 2D and 3D.
#[derive(Debug)]
pub struct Rotate;

impl BuiltinWorkbenchDefinition for Rotate {
    fn id() -> &'static str {
        "rotate"
    }

    fn kind() -> WorkbenchKind {
        WorkbenchKind::Operation
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|args| {
            Ok(BuiltinWorkpieceOutput::Transform(
                AffineTransform::Rotation(args.get("matrix")?),
            ))
        }
    }

    fn parameters() -> ParameterValueList {
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
