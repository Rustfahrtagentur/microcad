// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Call related evaluation entities

#[macro_use]
mod argument;
mod call_method;
mod call_trait;

pub use call_method::*;
pub use call_trait::*;

use crate::{eval::*, syntax::*, value::*};

use thiserror::Error;

impl Eval<ArgumentValueList> for ArgumentList {
    /// Evaluate into a [`ArgumentValueList`].
    fn eval(&self, context: &mut EvalContext) -> EvalResult<ArgumentValueList> {
        self.iter()
            .map(|arg| {
                (
                    arg.id.clone().unwrap_or(Identifier::none()),
                    arg.eval_value(context),
                )
            })
            .map(|(id, arg)| match arg {
                Ok(arg) => Ok((id.clone(), arg)),
                Err(err) => Err(err),
            })
            .collect()
    }
}

impl Eval for Call {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        // find self in symbol table by own name
        let symbol = match context.lookup(&self.name) {
            Ok(symbol) => symbol,
            Err(err) => {
                context.error(self, err)?;
                return Ok(Value::None);
            }
        };

        // evaluate arguments
        let args = match self.argument_list.eval(context) {
            Ok(args) => args,
            Err(err) => {
                // For builtin calls ONLY: If arguments cannot be evaluated put
                // the native argument code into a ArgumentValueList.
                // E.g. this is needed to give assert_valid() a qualified name.
                if symbol.with_def(|def| matches!(def, SymbolDefinition::Builtin(..))) {
                    self.argument_list
                        .iter()
                        .map(|arg| match context.source_code(&arg.expression) {
                            Ok(code) => Ok((
                                arg.id.clone().unwrap_or(Identifier::none()),
                                ArgumentValue::new(
                                    code.into(),
                                    arg.id.clone(),
                                    arg.src_ref.clone(),
                                ),
                            )),
                            Err(err) => Err(err),
                        })
                        .collect::<EvalResult<ArgumentValueList>>()?
                } else {
                    Err(err)?
                }
            }
        };

        log::debug!(
            "{call} {name:?}({args:?})",
            name = self.name,
            call = crate::mark!(CALL),
        );

        match context.scope(
            StackFrame::Call {
                symbol: symbol.clone(),
                args: args.clone(),
                src_ref: self.src_ref(),
            },
            |context| {
                symbol.with_def(|def| match def {
                    SymbolDefinition::Builtin(f) => f.call(&args, context),
                    SymbolDefinition::Workbench(w) => {
                        if matches!(*w.kind, WorkbenchKind::Operation) {
                            context.error(self, EvalError::CannotCallOperationWithoutWorkpiece)?;
                            Ok(Value::None)
                        } else {
                            Ok(Value::Model(w.call(
                                self.src_ref(),
                                symbol.clone(),
                                &args,
                                context,
                            )?))
                        }
                    }
                    SymbolDefinition::Function(f) => f.call(&args, context),
                    _ => {
                        context.error(self, EvalError::SymbolCannotBeCalled(symbol.full_name()))?;
                        Ok(Value::None)
                    }
                })
            },
        ) {
            Ok(value) => Ok(value),
            Err(err) => {
                context.error(self, err)?;
                Ok(Value::None)
            }
        }
    }
}

/// An error that occurred when looking for matching arguments between a call and a parameter definition.
#[derive(Error, Debug)]
pub enum MatchError {
    /// Duplicated argument.
    #[error("Duplicated argument: {0}")]
    DuplicatedArgument(Identifier),
    /// Occurs when a parameter was given in a call but not in the definition.
    #[error("Parameter `{0}` is not defined.")]
    ParameterNotDefined(Identifier),
    /// Mismatching type.
    #[error("Type mismatch for parameter `{0}`: expected `{1}`, got {2}")]
    PositionalArgumentTypeMismatch(Identifier, Type, Type),
    /// Parameter required by definition but given in the call.
    #[error("Missing parameter: {0}")]
    MissingParameter(Identifier),
}
