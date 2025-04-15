// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
use crate::builtin_namespace;
use microcad_lang::syntax::*;
use microcad_lang::{diag::*, eval::*, resolve::*, src_ref::*, value::*};

pub fn assert() -> SymbolNodeRcMut {
    SymbolNode::new_builtin_fn("assert".into(), &|args, context| {
        if let Ok(arg) = args.get_single() {
            if !arg.eval_bool(context)? {
                context.error(
                    arg.src_ref(),
                    Box::new(EvalError::AssertionFailed(format!("{arg}"))),
                )?;
            }
        } else {
            context.error(args.src_ref(), EvalError::NotAName(args.src_ref()))?;
        }
        Ok(Value::None)
    })
}

pub fn assert_valid() -> SymbolNodeRcMut {
    SymbolNode::new_builtin_fn("assert_valid".into(), &|args, context| {
        if let Ok(name) = args.get_single() {
            let name = QualifiedName::try_from(name.value.to_string())?;
            if context.lookup(&name).is_err() {
                return Err(EvalError::SymbolNotFound(name));
            }
        }
        Ok(Value::None)
    })
}

pub fn assert_invalid() -> SymbolNodeRcMut {
    SymbolNode::new_builtin_fn("assert_invalid".into(), &|args, context| {
        if let Ok(name) = args.get_single() {
            let name = QualifiedName::try_from(name.value.to_string())?;
            if context.lookup(&name).is_ok() {
                return Err(EvalError::SymbolFound(name));
            }
        }
        Ok(Value::None)
    })
}

#[test]
fn assert_ok() {
    let source_file = SourceFile::load("../tests/test_cases/syntax/assert_ok.µcad")
        .expect("cannot load test file");

    let mut context =
        EvalContext::from_source_file(source_file.clone(), builtin_namespace(), vec![]);
    context.add_symbol(super::builtin_namespace());

    assert!(source_file.eval(&mut context).is_ok());
}

#[test]
fn assert_fail() {
    use log::trace;
    microcad_lang::env_logger_init();

    let source_file = SourceFile::load("../tests/test_cases/syntax/assert_fail.µcad")
        .expect("cannot load test file");
    let mut context =
        EvalContext::from_source_file(source_file.clone(), builtin_namespace(), vec![]);
    context.add_symbol(super::builtin_namespace());
    let node = source_file.resolve(None);
    trace!("Source File Node:\n{node}");
    //trace!("Symbol Map:\n{}", context.symbols);

    assert!(source_file.eval(&mut context).is_ok());
    assert!(context.diag_handler().error_count > 0);

    println!(
        "{}",
        context
            .diag_handler()
            .pretty_print_to_string(&context)
            .expect("internal test error")
    );
    assert_eq!(
        context
            .diag_handler()
            .pretty_print_to_string(&context)
            .expect("internal test error"),
        "error: Assertion failed: false
  ---> ../tests/test_cases/syntax/assert_fail.µcad:1:19
     |
   1 | __builtin::assert(false);
     |                   ^^^^^
     |
"
    );
}
