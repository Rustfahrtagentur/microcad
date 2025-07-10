// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::str::FromStr;

#[cfg(test)]
use crate::builtin_module;
use microcad_lang::{diag::*, eval::*, parameter, resolve::*, syntax::*, ty::Ty, value::*};

pub fn assert() -> Symbol {
    Symbol::new_builtin(
        Identifier::no_ref("assert"),
        Some(
            [
                parameter!(v),                               // Parameter with any type
                parameter!(message: String = String::new()), // Optional message
            ]
            .into_iter()
            .collect(),
        ),
        &|params, args, context| {
            match ArgumentMap::find_match(args, params.expect("ParameterList")) {
                Ok(arg_map) => {
                    let v = arg_map[&"v".try_into()?].try_bool()?;
                    if !v {
                        let message = arg_map[&"message".try_into()?].try_string()?;
                        context.error(
                            arg_map,
                            EvalError::AssertionFailed(if message.is_empty() {
                                format!("{v}")
                            } else {
                                format!("{v}: {message}")
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

pub fn assert_eq() -> Symbol {
    Symbol::new_builtin(
        Identifier::no_ref("assert_eq"),
        Some(
            [
                parameter!(a),                               // Parameter with any type
                parameter!(b),                               // Parameter with any type
                parameter!(message: String = String::new()), // Optional message
            ]
            .into_iter()
            .collect(),
        ),
        &|params, args, context| {
            match ArgumentMap::find_match(args, params.expect("ParameterList")) {
                Ok(arg_map) => {
                    let a_value = &arg_map[&"a".try_into()?];
                    let b_value = &arg_map[&"b".try_into()?];
                    if a_value != b_value {
                        let message = arg_map[&"message".try_into()?].try_string()?;
                        context.error(
                            args,
                            EvalError::AssertionFailed(if message.is_empty() {
                                format!("{a_value} != {b_value}")
                            } else {
                                format!("{a_value} != {b_value}: {message}")
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

pub fn type_of() -> Symbol {
    let id = Identifier::from_str("type_of").expect("valid id");
    Symbol::new_builtin(id, None, &|_, args, _| {
        if let Ok((_, arg)) = args.get_single() {
            let ty = arg.value.ty();
            return Ok(Value::String(ty.to_string()));
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
