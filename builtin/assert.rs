// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::str::FromStr;

#[cfg(test)]
use crate::builtin_module;
use microcad_lang::{diag::*, eval::*, parameter_value, resolve::*, syntax::*, value::*};

pub fn assert() -> Symbol {
    let id = Identifier::no_ref("assert");
    Symbol::new_builtin(
        id,
        Some(
            vec![
                parameter_value!(x),                               // Parameter with any type
                parameter_value!(message: String = String::new()), // Optional message
            ]
            .into(),
        ),
        &|params, args, context| {
            match ArgumentMap::find_match(args, params.expect("ParameterList")) {
                Ok(arg_map) => {
                    if !arg_map[&Identifier::no_ref("x")].try_bool()? {
                        let message = arg_map[&Identifier::no_ref("message")].try_string()?;
                        let arg = args.first().expect("At least one argument");
                        context.error(
                            arg,
                            EvalError::AssertionFailed(if message.is_empty() {
                                format!("{arg}")
                            } else {
                                format!("{arg}: {message}")
                            }),
                        )?;
                    }
                }
                Err(err) => {
                    // Called `assert` with no or more than 2 parameters
                    context.error(args, err)?
                }
            }

            Ok(Value::None)
        },
    )
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
