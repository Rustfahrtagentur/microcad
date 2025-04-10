// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin print method

use microcad_lang::{eval::*, rc_mut::*, resolve::*, value::*};

pub fn print() -> RcMut<SymbolNode> {
    SymbolNode::new_builtin_fn("print".into(), &|args, context| {
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
    use microcad_lang::syntax::*;

    let source_file =
        SourceFile::load("../tests/test_cases/print.µcad").expect("cannot load test file");

    let mut context = EvalContext::from_source_file(source_file.clone(), vec![], None);
    context.add_symbol(super::builtin_namespace());

    assert!(source_file.eval(&mut context).is_ok());
}
