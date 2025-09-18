// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::str::FromStr;

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
                        let a_value = &a.get_value("a").expect("missing parameter");

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
                        } else {
                            let message: String = a.get("message");
                            context.error(
                                args,
                                EvalError::AssertionFailed(if message.is_empty() {
                                    format!("Invalid: {a_value}")
                                } else {
                                    "{message}".to_string()
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

pub fn assert_valid() -> Symbol {
    let id = Identifier::from_str("assert_valid").expect("valid id");
    Symbol::new_builtin(id, None, &|_, args, context| {
        if let Ok((_, arg)) = args.get_single() {
            if let Ok(name) = QualifiedName::try_from(arg.value.to_string()) {
                match context.lookup(&name) {
                    Ok(symbol) => {
                        if let Ok(value) = symbol.get_value() {
                            if value.is_invalid() {
                                context.error(
                                    &arg,
                                    EvalError::AssertionFailed(format!("invalid value: {value}")),
                                )?;
                            }
                        }
                    }
                    Err(err) => context.error(&arg, err)?,
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
                    if let Ok(value) = symbol.get_value() {
                        if !value.is_invalid() {
                            context.error(&arg, EvalError::SymbolFound(symbol.full_name()))?
                        }
                    }
                }
            }
        }
        Ok(Value::None)
    })
}
