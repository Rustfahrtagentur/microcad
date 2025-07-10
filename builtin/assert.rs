// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::str::FromStr;

#[cfg(test)]
use crate::builtin_module;
use microcad_lang::{diag::*, eval::*, parameter, resolve::*, syntax::*, value::*};

pub fn assert() -> Symbol {
    Symbol::new_builtin(
        Identifier::no_ref("assert"),
        Some(
            [
                parameter!(v : Bool),                        // Parameter with any type
                parameter!(message: String = String::new()), // Optional message
            ]
            .into_iter()
            .collect(),
        ),
        &|params, args, context| {
            match ArgumentMatch::find_multi_match(args, params.expect("parameter list")) {
                Ok(multi_args) => {
                    for a in multi_args {
                        let v: bool = a.get("v");
                        if !v {
                            let message: String = a.get("message");
                            context.error(
                                args,
                                EvalError::AssertionFailed(if message.is_empty() {
                                    format!("{v}")
                                } else {
                                    format!("{v}: {message}")
                                }),
                            )?;
                        }
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

fn all_equal<T: PartialEq + std::fmt::Debug>(mut iter: impl Iterator<Item = T>) -> bool {
    if let Some(first) = iter.next() {
        iter.all(|x| x == first)
    } else {
        // Wenn der Iterator leer ist, gibt es keine Elemente zum Vergleichen.
        true
    }
}

pub fn assert_eq() -> Symbol {
    Symbol::new_builtin(
        Identifier::no_ref("assert_eq"),
        Some(
            [
                parameter!(a),                               // Parameter with any type
                parameter!(message: String = String::new()), // Optional message
            ]
            .into_iter()
            .collect(),
        ),
        &|params, args, context| {
            match ArgumentMatch::find_multi_match(args, params.expect("ParameterList")) {
                Ok(multi_args) => {
                    for a in multi_args {
                        let a_value = &a.get_value("a");

                        if let Value::Array(exprs) = a_value {
                            if !all_equal(exprs.iter()) {
                                let message: String = a.get("message");
                                context.error(
                                    args,
                                    EvalError::AssertionFailed(if message.is_empty() {
                                        format!("Values differ: {exprs}")
                                    } else {
                                        "{message}".to_string()
                                    }),
                                )?;
                            }
                        }
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
    Symbol::new_builtin(id, None, &|_, args, context| {
        if let Ok((_, arg)) = args.get_single() {
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
    Symbol::new_builtin(id, None, &|_, args, context| {
        if let Ok((_, arg)) = args.get_single() {
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
