// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::{
    eval::{BuiltinFunction, EvalError},
    function_signature, parameter, parameter_list,
    parse::{Expression, GetSourceFileByHash},
    src_ref::SrcReferrer,
};

pub fn builtin_fn() -> BuiltinFunction {
    BuiltinFunction::new(
        "assert".into(),
        function_signature!(parameter_list![
            parameter!(condition: Bool),
            parameter!(message: String = "Assertion failed")
        ]),
        &|args, ctx| {
            let message: String = if let Some(m) = args.get("message") {
                m.clone().try_into()?
            } else {
                return Err(EvalError::GrammarRuleError("cannot fetch `message`".into()));
            };

            let condition: bool = args["condition"].clone().try_into()?;
            if !condition {
                if let Some(condition_src) = ctx.get_source_string(args["condition"].src_ref()) {
                    ctx.error_with_stack_trace(
                        args.src_ref(),
                        EvalError::AssertionFailedWithCondition(message, condition_src.into()),
                    )?;
                } else {
                    ctx.error_with_stack_trace(
                        args.src_ref(),
                        EvalError::AssertionFailed(message),
                    )?;
                }
            }
            Ok(None)
        },
    )
}
