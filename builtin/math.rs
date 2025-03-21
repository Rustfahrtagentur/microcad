// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::{diag::*, src_ref::*, ty::*, SymbolNode, *};

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
    crate::NamespaceBuilder::new("math").symbol(abs()).build()
}
