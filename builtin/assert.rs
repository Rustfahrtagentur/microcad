// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::str::FromStr;

#[cfg(test)]
use crate::builtin_module;
use microcad_lang::{diag::*, eval::*, resolve::*, syntax::*, value::*};

pub fn assert() -> Symbol {
    let id = Identifier::no_ref("assert");
    Symbol::new_builtin(id, None, &|_params, args, context| {
        match args.len() {
            // assert(false)
            1 => {
                if let Ok(arg) = args.get_single() {
                    if !arg.value.try_bool()? {
                        context.error(arg, EvalError::AssertionFailed(format!("{arg}")))?;
                    }
                }
            }
            // assert(false, "A message that is shown when assertion failed")
            2 => {
                let (assertion, message) = (&args[0], &args[1].value);
                if !assertion.value.try_bool()? {
                    context.error(
                        args,
                        EvalError::AssertionFailed(format!("{assertion}: {message}")),
                    )?;
                }
            }
            // Called `assert` with no or more than 2 parameters
            _ => context.error(args, EvalError::AssertWrongSignature(args.clone()))?,
        }
        Ok(Value::None)
    })
}

pub fn assert_valid() -> Symbol {
    let id = Identifier::from_str("assert_valid").expect("valid id");
    Symbol::new_builtin(id, None, &|_params, args, context| {
        if let Ok(arg) = args.get_single() {
            if let Ok(name) = QualifiedName::try_from(arg.value.to_string()) {
                if let Err(err) = context.lookup(&name) {
                    context.error(arg, err)?;
                }
            }
        }
        Ok(Value::None)
    })
}

pub fn assert_invalid() -> Symbol {
    let id = Identifier::from_str("assert_invalid").expect("valid id");
    Symbol::new_builtin(id, None, &|_params, args, context| {
        if let Ok(arg) = args.get_single() {
            if let Ok(name) = QualifiedName::try_from(arg.value.to_string()) {
                if let Ok(symbol) = context.lookup(&name) {
                    context.error(name, EvalError::SymbolFound(symbol.full_name()))?;
                }
            }
        }
        Ok(Value::None)
    })
}

#[test]
fn assert_ok() {
    let mut context = Context::from_source(
        "../tests/test_cases/syntax/assert_ok.µcad",
        builtin_module(),
        &[],
    )
    .expect("resolvable file ../tests/test_cases/syntax/assert_ok.µcad");

    assert!(context.eval().is_ok());
}

#[test]
fn assert_fail() {
    let mut context = Context::from_source(
        "../tests/test_cases/syntax/assert_fail.µcad",
        builtin_module(),
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
