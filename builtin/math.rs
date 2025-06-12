// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::{diag::*, eval::*, resolve::*, syntax::*, ty::*, value::*};
use std::str::FromStr;

/// Absolute value abs(x)
fn abs() -> Symbol {
    let id = Identifier::no_ref("abs");
    Symbol::new_builtin(id, &|args, ctx| {
        let arg = args.get_single()?;
        Ok(match &arg.value {
            Value::Integer(i) => Value::Integer(i.abs()),
            value => {
                ctx.error(
                    arg,
                    EvalError::ParameterTypeMismatch {
                        id: arg.id.clone().unwrap_or(Identifier::no_ref("x")),
                        expected: Type::Integer,
                        found: value.ty(),
                    },
                )?;
                Value::None
            }
        })
    })
}

pub fn math() -> Symbol {
    crate::ModuleBuilder::new("math".try_into().expect("unexpected name error"))
        .symbol(Symbol::new_constant(
            Identifier::no_ref("PI"),
            Value::Scalar(std::f64::consts::PI),
        ))
        .symbol(abs())
        .build()
}
