// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin print method

use std::sync::Mutex;

use lazy_static::lazy_static;
use microcad_lang::*;

lazy_static! {
    /// Alternate print output buffer
    pub static ref output: Mutex<String> = Mutex::new(String::new());
}

/// Build builtin assert symbols
pub fn build(builtin_symbol: &mut RcMut<SymbolNode>) {
    SymbolNode::insert_child(builtin_symbol, print());
}

fn print() -> RcMut<SymbolNode> {
    SymbolNode::new_builtin_fn("print".into(), &|args, context| {
        args.iter().try_for_each(|arg| -> Result<(), EvalError> {
            let value = arg.value.eval(context)?;
            println!("{value}");
            Ok(())
        })?;
        Ok(Value::None)
    })
}

#[test]
fn assert_ok() {
    let source_file =
        SourceFile::load("../tests/test_cases/print.µcad").expect("cannot load test file");

    let mut context = EvalContext::from_source_file(source_file.clone());
    context.add_symbol(super::builtin_namespace());

    assert!(source_file.eval(&mut context).is_ok());
}
