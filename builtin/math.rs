// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::{diag::*, eval::*, resolve::*, src_ref::*, syntax::*, ty::*, value::*};
use std::str::FromStr;

/// Absolute value abs(x)
fn abs() -> Symbol {
    let id = Identifier::no_ref("abs");
    Symbol::new_builtin(id, &|args, ctx| {
        let arg = args.get_single()?;
        Ok(match arg.value.eval(ctx)? {
            Value::Integer(i) => Value::Integer(Refer::new(i.abs(), arg.src_ref())),
            value => {
                ctx.error(
                    arg,
                    EvalError::ParameterTypeMismatch {
                        id: arg.name.clone().unwrap_or(Identifier::no_ref("x")),
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
    crate::NamespaceBuilder::new("math".try_into().expect("unexpected name error"))
        .symbol(Symbol::new_constant(
            Identifier::from_str("pi").expect("valid id"),
            Value::Scalar(Refer::none(std::f64::consts::PI)),
        ))
        .symbol(abs())
        .build()
}
