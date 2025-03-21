// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::{diag::*, eval::*, resolve::*, src_ref::*, ty::*, value::*, RcMut};

// Absolute value abs(x)
fn abs() -> RcMut<SymbolNode> {
    SymbolNode::new_builtin_fn("abs".into(), &|args, ctx| {
        let arg = args.get_single()?;
        Ok(match arg.value.eval(ctx)? {
            Value::Integer(i) => Value::Integer(Refer::new(i.abs(), arg.src_ref())),
            value => {
                ctx.error(arg, EvalError::InvalidType(value.ty()))?;
                Value::None
            }
        })
    })
}

pub fn math() -> RcMut<SymbolNode> {
    crate::NamespaceBuilder::new("math")
        .symbol(SymbolNode::new_builtin_constant(
            "pi",
            Value::Scalar(Refer::none(std::f64::consts::PI)),
        ))
        .symbol(abs())
        .build()
}
