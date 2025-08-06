// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::{
    eval::*,
    model::*,
    syntax::Identifier,
    ty::{MatrixType, Type},
    value::*,
};

/// Builtin definition for a 2D circle
#[derive(Debug)]
pub struct Rotate;

impl BuiltinWorkbenchDefinition for Rotate {
    fn id() -> &'static str {
        "rotate"
    }

    fn model(args: &Tuple) -> EvalResult<Model> {
        Ok(ModelBuilder::new_transform(AffineTransform::Rotation(args.get("matrix")?)).build())
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
