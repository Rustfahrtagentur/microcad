// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
use crate::builtin_namespace;
use microcad_lang::{diag::*, eval::*, resolve::*, src_ref::*, syntax::*, value::*};

pub fn assert() -> SymbolNodeRcMut {
    SymbolNode::new_builtin_fn("assert".into(), &|args, context| {
        if let Ok(arg) = args.get_single() {
            if !arg.eval_bool(context)? {
                context.error(arg.src_ref(), EvalError::AssertionFailed(format!("{arg}")))?;
            }
        } else {
            context.error(
                args.src_ref(),
                EvalError::ArgumentCountMismatch {
                    args: args.clone(),
                    expected: 1,
                    found: args.len(),
                },
            )?;
        }
        Ok(Value::None)
    })
}

pub fn assert_valid() -> SymbolNodeRcMut {
    SymbolNode::new_builtin_fn("assert_valid".into(), &|args, context| {
        if let Ok(name) = args.get_single() {
            let name = QualifiedName::try_from(name.value.to_string())?;
            if let Err(err) = context.lookup(&name) {
                context.error(name.clone(), err)?;
            }
        }
        Ok(Value::None)
    })
}

pub fn assert_invalid() -> SymbolNodeRcMut {
    SymbolNode::new_builtin_fn("assert_invalid".into(), &|args, context| {
        if let Ok(name) = args.get_single() {
            let name = QualifiedName::try_from(name.value.to_string())?;
            if let Err(err) = context.lookup(&name) {
                context.error(name.clone(), err)?;
            }
        }
        Ok(Value::None)
    })
}

#[test]
fn assert_ok() {
    microcad_lang::env_logger_init();

    let mut context = EvalContext::from_source(
        "../tests/test_cases/syntax/assert_ok.µcad",
        builtin_namespace(),
        &[],
    )
    .expect("resolvable file ../tests/test_cases/syntax/assert_ok.µcad");

    assert!(context.eval().is_ok());
}

#[test]
fn assert_fail() {
    microcad_lang::env_logger_init();

    let mut context = EvalContext::from_source(
        "../tests/test_cases/syntax/assert_fail.µcad",
        builtin_namespace(),
        &[],
    )
    .expect("resolvable file ../tests/test_cases/syntax/assert_fail.µcad");

    assert!(context.eval().is_ok());
    assert!(context.diag_handler().error_count > 0);

    assert_eq!(
        context
            .diag_handler()
            .pretty_print_to_string(&context)
            .expect("internal test error"),
        "error: Assertion failed: false
  ---> ../tests/test_cases/syntax/assert_fail.µcad:1:19
     |
   1 | __builtin::assert(false);
     |                   ^^^^^
     |
"
    );
}
