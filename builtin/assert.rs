// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::{diag::*, eval::*, src_ref::*, ty::*, *};

/// Build builtin assert symbols
pub fn build(builtin_symbol: &mut RcMut<SymbolNode>) {
    SymbolNode::insert_child(builtin_symbol, assert());
    SymbolNode::insert_child(builtin_symbol, assert_valid());
    SymbolNode::insert_child(builtin_symbol, assert_invalid());
}

fn assert() -> RcMut<SymbolNode> {
    SymbolNode::new_builtin_fn("assert".into(), &|args, context| {
        if let Ok(arg) = args.get_single() {
            match arg.value.eval(context) {
                Ok(Value::Bool(cond)) => {
                    if !cond.value {
                        context.error(
                            arg.src_ref(),
                            Box::new(EvalError::AssertionFailed(format!("{arg}"))),
                        )?;
                    }
                }
                Ok(value) => context.error(
                    arg.src_ref(),
                    Box::new(EvalError::InvalidArgumentType(value.ty().clone())),
                )?,
                Err(err) => return Err(err),
            }
        } else {
            context.error(args.src_ref(), EvalError::NotAName(args.src_ref()))?;
        }
        Ok(Value::None)
    })
}

fn assert_valid() -> RcMut<SymbolNode> {
    SymbolNode::new_builtin_fn("assert_valid".into(), &|args, context| match look_up(
        args.get_single()?,
        context,
    ) {
        Ok(LookUp::Local(_)) | Ok(LookUp::Symbol(_)) => Ok(Value::None),
        Ok(LookUp::NotFound(no_name)) => {
            context.error(SrcRef(None), EvalError::NotAName(no_name.src_ref()))?;
            Ok(Value::None)
        }
        Err(err) => Err(err),
    })
}

fn assert_invalid() -> RcMut<SymbolNode> {
    SymbolNode::new_builtin_fn("assert_invalid".into(), &|args, context| {
        match look_up(args.get_single()?, context) {
            Ok(LookUp::Local(name)) => {
                context.error(SrcRef(None), EvalError::LocalNotFound(name))?;
            }
            Ok(LookUp::Symbol(name)) => {
                context.error(SrcRef(None), EvalError::SymbolNotFound(name))?;
            }
            _ => (),
        };
        Ok(Value::None)
    })
}

fn look_up(arg: &CallArgument, context: &mut EvalContext) -> EvalResult<LookUp> {
    if let Expression::Nested(nested) = &arg.value {
        if let Some(name) = nested.single_qualified_name() {
            match context.look_up(&name) {
                LookUp::Symbol(name) => Ok(LookUp::Symbol(name)),
                LookUp::Local(id) => Ok(LookUp::Local(id)),
                _ => Err(EvalError::LookUpFailed(arg.value.clone())),
            }
        } else {
            Err(EvalError::NotAName(arg.value.src_ref()))
        }
    } else {
        Err(EvalError::NotAName(arg.value.src_ref()))
    }
}

#[test]
fn assert_ok() {
    use microcad_lang::*;
    let source_file = SourceFile::load("../tests/test_cases/syntax/assert_ok.µcad").expect("");

    let mut context = EvalContext::from_source_file(source_file.clone());
    context.add_symbol(super::builtin_module());

    assert!(source_file.eval(&mut context).is_ok());
}

#[test]
fn assert_fail() {
    use microcad_lang::*;
    let source_file = SourceFile::load("../tests/test_cases/syntax/assert_fail.µcad").expect("");

    let mut context = EvalContext::from_source_file(source_file.clone());
    context.add_symbol(super::builtin_module());

    assert!(source_file.eval(&mut context).is_ok());
    assert!(context.diag_handler().error_count > 0);

    println!(
        "{}",
        context
            .diag_handler()
            .pretty_print_to_string(source_file.as_ref())
            .expect("internal test error")
    );
    assert_eq!(
        context
            .diag_handler()
            .pretty_print_to_string(source_file.as_ref())
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
