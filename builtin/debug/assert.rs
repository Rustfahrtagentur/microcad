// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

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
                                    message
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
        true
    }
}

pub fn assert_eq() -> Symbol {
    Symbol::new_builtin(
        Identifier::no_ref("assert_eq"),
        Some(
            [
                parameter!(array),                           // Parameter with any type
                parameter!(message: String = String::new()), // Optional message
            ]
            .into_iter()
            .collect(),
        ),
        &|params, args, context| {
            match ArgumentMatch::find_multi_match(args, params.expect("ParameterList")) {
                Ok(multi_args) => {
                    for array in multi_args {
                        let array_value = &array.get_value("array").expect("missing parameter");

                        if let Value::Array(exprs) = array_value {
                            if !all_equal(exprs.iter()) {
                                let message: String = array.get("message");
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
                            let message: String = array.get("message");
                            context.error(
                                args,
                                EvalError::AssertionFailed(if message.is_empty() {
                                    format!("Invalid: {array_value}")
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
    Symbol::new_builtin(
        Identifier::no_ref("assert_valid"),
        Some(
            [
                parameter!(target: Target),                  // Parameter name
                parameter!(message: String = String::new()), // Optional message
            ]
            .into_iter()
            .collect(),
        ),
        &|params, args, context| {
            match ArgumentMatch::find_multi_match(args, params.expect("ParameterList")) {
                Ok(multi_args) => {
                    for arg in multi_args {
                        let target = arg.get::<Target>("target");
                        if target.target.is_none() {
                            context.error(
                                &arg,
                                EvalError::AssertionFailed(format!(
                                    "Symbol `{}` not found.",
                                    target.name
                                )),
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

pub fn assert_invalid() -> Symbol {
    Symbol::new_builtin(
        Identifier::no_ref("assert_invalid"),
        Some(
            [
                parameter!(target: Target),                  // Parameter name
                parameter!(message: String = String::new()), // Optional message
            ]
            .into_iter()
            .collect(),
        ),
        &|params, args, context| {
            match ArgumentMatch::find_multi_match(args, params.expect("ParameterList")) {
                Ok(multi_args) => {
                    for arg in multi_args {
                        let target = arg.get::<Target>("target");
                        if let Some(query) = target.target {
                            log::trace!("target_name: {query}, {}", target.name);
                            context.error(
                                &arg,
                                EvalError::AssertionFailed(format!(
                                    "Found valid symbol '{name}' within module '{base}'.",
                                    name = target.name,
                                    base = query.base(&target.name)
                                )),
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
