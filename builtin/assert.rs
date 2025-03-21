// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::{eval::*, src_ref::*, *};

/// Build builtin assert symbols
pub fn build(builtin_symbol: &mut RcMut<SymbolNode>) {
    SymbolNode::insert_child(builtin_symbol, assert_valid());
    SymbolNode::insert_child(builtin_symbol, assert_invalid());
}

fn assert_valid() -> RcMut<SymbolNode> {
    SymbolNode::new_builtin_fn(
        "assert_valid".into(),
        &|args, context| match look_up(args, context) {
            Ok(LookUp::Local(_)) | Ok(LookUp::Symbol(_)) => Ok(Value::None),
            Ok(LookUp::NotFound(no_name)) => {
                context
                    .error_with_stack_trace(SrcRef(None), EvalError::NotAName(no_name.src_ref()))?;
                Ok(Value::None)
            }
            Err(err) => Err(err),
        },
    )
}

fn assert_invalid() -> RcMut<SymbolNode> {
    SymbolNode::new_builtin_fn("assert_invalid".into(), &|args, context| {
        match look_up(args, context) {
            Ok(LookUp::Local(name)) => {
                context.error_with_stack_trace(SrcRef(None), EvalError::LocalNotFound(name))?;
            }
            Ok(LookUp::Symbol(name)) => {
                context.error_with_stack_trace(SrcRef(None), EvalError::SymbolNotFound(name))?;
            }
            _ => (),
        };

        Ok(Value::None)
    })
}

fn look_up(args: &CallArgumentList, context: &mut EvalContext) -> EvalResult<LookUp> {
    if args.len() != 1 {
        return Err(EvalError::ArgumentCountMismatch {
            args: args.clone(),
            expected: 1,
            found: args.len(),
        });
    }
    if let Some(first) = args.first() {
        if let Expression::Nested(nested) = &first.value {
            if let Some(name) = nested.single_qualified_name() {
                match context.look_up(&name) {
                    LookUp::Symbol(name) => return Ok(LookUp::Symbol(name)),
                    LookUp::Local(id) => return Ok(LookUp::Local(id)),
                    _ => (),
                }
            } else {
                return Err(EvalError::NotAName(first.value.src_ref()));
            }
        } else {
            return Err(EvalError::NotAName(first.value.src_ref()));
        }
        return Err(EvalError::LookUpFailed(first.value.clone()));
    }
    unreachable!()
}
