// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::{diag::*, eval::*, resolve::*, syntax::*, value::*};

/// Module for built-logging.
pub fn log() -> Symbol {
    crate::ModuleBuilder::new("log".try_into().expect("valid id"))
        .symbol(todo())
        .symbol(error())
        .symbol(warning())
        .symbol(info())
        .build()
}

pub fn error() -> Symbol {
    Symbol::new_builtin(
        Identifier::no_ref("error"),
        None,
        &|_params, args, context| {
            args.iter()
                .try_for_each(|(_, arg)| -> Result<(), EvalError> {
                    context.error(
                        args,
                        EvalError::BuiltinError(format!("{value}", value = arg.value)),
                    )
                })?;
            Ok(Value::None)
        },
    )
}

pub fn warning() -> Symbol {
    Symbol::new_builtin(
        Identifier::no_ref("warning"),
        None,
        &|_params, args, context| {
            args.iter()
                .try_for_each(|(_, arg)| -> Result<(), EvalError> {
                    context.warning(
                        args,
                        EvalError::BuiltinError(format!("{value}", value = arg.value)),
                    )
                })?;
            Ok(Value::None)
        },
    )
}

pub fn info() -> Symbol {
    Symbol::new_builtin(
        Identifier::no_ref("info"),
        None,
        &|_params, args, context| {
            args.iter()
                .try_for_each(|(_, arg)| -> Result<(), EvalError> {
                    context.info(args, format!("{value}", value = arg.value));
                    Ok(())
                })?;
            Ok(Value::None)
        },
    )
}

pub fn todo() -> Symbol {
    Symbol::new_builtin(
        Identifier::no_ref("todo"),
        None,
        &|_params, args, context| {
            args.iter()
                .try_for_each(|(_, arg)| -> Result<(), EvalError> {
                    context.error(args, EvalError::Todo(format!("{value}", value = arg.value)))
                })?;
            Ok(Value::None)
        },
    )
}
