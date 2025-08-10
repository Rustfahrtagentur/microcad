// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin print method

use microcad_lang::{eval::*, resolve::*, syntax::*, value::*};

pub fn print() -> Symbol {
    Symbol::new_builtin(
        Identifier::no_ref("print"),
        None,
        &|_params, args, context| {
            args.iter()
                .try_for_each(|(_, arg)| -> Result<(), EvalError> {
                    context.print(format!("{value}", value = arg.value));
                    Ok(())
                })?;
            Ok(Value::None)
        },
    )
}

#[test]
fn print_test() {
    let mut context = Context::from_source(
        "../tests/test_cases/print.µcad",
        crate::builtin_module(),
        &[],
    )
    .expect("resolvable file ../tests/test_cases/print.µcad");

    assert!(context.eval().is_ok());
}
