// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Call related evaluation entities

#[macro_use]
mod call_argument_value;
mod call_argument;
mod call_argument_value_list;
mod call_method;
mod call_trait;

pub use call_argument_value::*;
pub use call_argument_value_list::*;
pub use call_method::*;
pub use call_trait::*;

use crate::{eval::*, syntax::*};

use thiserror::Error;

impl Eval<CallArgumentValueList> for CallArgumentList {
    /// Evaluate into a [`CallArgumentValueList`].
    fn eval(&self, context: &mut Context) -> EvalResult<CallArgumentValueList> {
        let mut v = CallArgumentValueList::default();
        for call_arg in self.iter() {
            v.push(call_arg.eval_value(context)?)
                .expect("Could not insert call argument value");
        }
        Ok(v)
    }
}

impl Eval for Call {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
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
                // builtin functions get their argument list as string if it can't be evaluated
                if let SymbolDefinition::Builtin(_) = &symbol.borrow().def {
                    CallArgumentValueList::from_code(
                        context.source_code(&self.argument_list)?,
                        &self.argument_list,
                    )
                } else {
                    return Err(err);
                }
            }
        };

        match context.scope(
            StackFrame::Call {
                symbol: symbol.clone(),
                args: args.clone(),
                src_ref: self.src_ref(),
            },
            |context| match &symbol.borrow().def {
                SymbolDefinition::Builtin(f) => f.call(&args, context),
                SymbolDefinition::Part(m) => Ok(Value::Nodes(m.call(&args, context)?)),
                SymbolDefinition::Function(f) => f.call(&args, context),
                _ => {
                    context.error(
                        self,
                        EvalError::Todo(format!(
                            "cannot evaluate call of {} at {}",
                            self,
                            context.locate(self)?
                        )),
                    )?;
                    Ok(Value::None)
                }
            },
        ) {
            Ok(Value::Nodes(nodes)) => {
                // Store the information, saying that these nodes have been created by this symbol.
                nodes.set_creator(symbol, self.src_ref());

                Ok(Value::Nodes(nodes))
            }
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
