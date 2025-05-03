// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::{
    diag::*, eval::*, resolve::*, src_ref::*, syntax::Identifier, ty::*, value::*,
};

// Absolute value abs(x)
fn abs() -> SymbolNodeRcMut {
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

pub fn math() -> SymbolNodeRcMut {
    crate::NamespaceBuilder::new("math".try_into().expect("unexpected name error"))
        .symbol(SymbolNode::new_constant(
            Identifier(Refer::none("pi".into())),
            Value::Scalar(Refer::none(std::f64::consts::PI)),
        ))
        .symbol(abs())
        .build()
}
