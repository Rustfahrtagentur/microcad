// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin print method

use microcad_lang::{eval::*, resolve::*, syntax::*, value::*};
use std::str::FromStr;

pub fn print() -> Symbol {
    let id = Identifier::from_str("print").expect("valid id");
    Symbol::new_builtin_fn(id, &|args, context| {
        args.iter().try_for_each(|arg| -> Result<(), EvalError> {
            let value = arg.value.eval(context)?;
            context.print(format!("{value}"));
            Ok(())
        })?;
        Ok(Value::None)
    })
}

#[test]
fn assert_ok() {
    microcad_lang::env_logger_init();

    let mut context = Context::from_source(
        "../tests/test_cases/print.µcad",
        crate::builtin_namespace(),
        &[],
    )
    .expect("resolvable file ../tests/test_cases/print.µcad");

    assert!(context.eval().is_ok());
}
