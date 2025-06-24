// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin print method

use microcad_lang::{diag::*, eval::*, resolve::*, syntax::*, value::*};
use std::str::FromStr;

pub fn print() -> Symbol {
    let id = Identifier::from_str("print").expect("valid id");
    Symbol::new_builtin(id, None, &|_params, args, context| {
        args.iter().try_for_each(|arg| -> Result<(), EvalError> {
            context.print(format!("{value}", value = arg.value));
            Ok(())
        })?;
        Ok(Value::None)
    })
}

pub fn error() -> Symbol {
    let id = Identifier::from_str("error").expect("valid id");
    Symbol::new_builtin(id, None, &|_params, args, context| {
        args.iter().try_for_each(|arg| -> Result<(), EvalError> {
            context.error(
                args,
                EvalError::BuiltinError(format!("{value}", value = arg.value)),
            )
        })?;
        Ok(Value::None)
    })
}

pub fn warning() -> Symbol {
    let id = Identifier::from_str("warning").expect("valid id");
    Symbol::new_builtin(id, None, &|_params, args, context| {
        args.iter().try_for_each(|arg| -> Result<(), EvalError> {
            context.error(
                args,
                EvalError::BuiltinError(format!("{value}", value = arg.value)),
            )
        })?;
        Ok(Value::None)
    })
}

#[test]
fn assert_ok() {
    let mut context = Context::from_source(
        "../tests/test_cases/print.µcad",
        crate::builtin_module(),
        &[],
    )
    .expect("resolvable file ../tests/test_cases/print.µcad");

    assert!(context.eval().is_ok());
}
