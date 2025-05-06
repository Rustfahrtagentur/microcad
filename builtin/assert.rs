// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::str::FromStr;

#[cfg(test)]
use crate::builtin_namespace;
use microcad_lang::{diag::*, eval::*, resolve::*, src_ref::*, syntax::*, value::*};

pub fn assert() -> Symbol {
    let id = Identifier::from_str("assert").expect("valid id");
    Symbol::new_builtin(id, &|args, context| {
        if let Ok(arg) = args.get_single() {
            if !arg.value.try_bool()? {
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

pub fn assert_valid() -> Symbol {
    let id = Identifier::from_str("assert_valid").expect("valid id");
    Symbol::new_builtin(id, &|args, context| {
        if let Ok(arg) = args.get_single() {
            let name = QualifiedName::try_from(arg.value.to_string())?;
            if let Err(err) = context.lookup(&name) {
                context.error(arg, err)?;
            }
        }
        Ok(Value::None)
    })
}

pub fn assert_invalid() -> Symbol {
    let id = Identifier::from_str("assert_invalid").expect("valid id");
    Symbol::new_builtin(id, &|args, context| {
        if let Ok(name) = args.get_single() {
            let n = QualifiedName::try_from(name.value.to_string())?;
            if let Ok(symbol) = context.lookup(&n) {
                context.error(name, EvalError::SymbolFound(symbol.full_name()))?;
            }
        }
        Ok(Value::None)
    })
}

#[test]
fn assert_ok() {
    let mut context = Context::from_source(
        "../tests/test_cases/syntax/assert_ok.µcad",
        builtin_namespace(),
        &[],
    )
    .expect("resolvable file ../tests/test_cases/syntax/assert_ok.µcad");

    assert!(context.eval().is_ok());
}

#[test]
fn assert_fail() {
    let mut context = Context::from_source(
        "../tests/test_cases/syntax/assert_fail.µcad",
        builtin_namespace(),
        &[],
    )
    .expect("resolvable file ../tests/test_cases/syntax/assert_fail.µcad");

    assert!(context.eval().is_ok());
    assert!(context.error_count() > 0);

    assert_eq!(
        context.diagnosis(),
        "error: Assertion failed: false
  ---> ../tests/test_cases/syntax/assert_fail.µcad:1:19
     |
   1 | __builtin::assert(false);
     |                   ^^^^^
     |
"
    );
}
