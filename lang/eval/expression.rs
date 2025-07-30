// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

impl Eval for ArrayExpression {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        let value_list = ValueList::new(
            self.list
                .iter()
                .map(|expr| expr.eval(context))
                .collect::<Result<_, _>>()?,
        )
        .bundle_unit(self.unit)?;

        match value_list.types().common_type() {
            Some(common_type) => Ok(Value::Array(Array::new(value_list, common_type))),
            None => {
                context.error(
                    self,
                    EvalError::ListElementsDifferentTypes(value_list.types()),
                )?;
                Ok(Value::None)
            }
        }
    }
}

impl Expression {
    /// Evaluate an expression together with an attribute list.
    ///
    /// The attribute list will be also evaluated and the resulting attributes
    /// will be assigned to the resulting value.
    pub fn eval_with_attribute_list(
        &self,
        attribute_list: &AttributeList,
        context: &mut Context,
    ) -> EvalResult<Value> {
        let value = self.eval(context)?;
        match value {
            Value::None => Ok(Value::None),
            _ => {
                if !attribute_list.is_empty() {
                    context.error(
                        attribute_list,
                        AttributeError::CannotAssignToExpression(self.clone().into()),
                    )?;
                }
                Ok(value)
            }
        }
    }
}

impl Eval for Expression {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        log::trace!("Evaluating expression:\n{self}");
        let result = match self {
            Self::Literal(literal) => literal.eval(context),
            Self::FormatString(format_string) => format_string.eval(context),
            Self::ArrayExpression(array_expression) => array_expression.eval(context),
            Self::TupleExpression(tuple_expression) => tuple_expression.eval(context),
            Self::BinaryOp {
                lhs,
                op,
                rhs,
                src_ref: _,
            } => {
                let lhs: Value = lhs.eval(context)?;
                let rhs: Value = rhs.eval(context)?;
                if lhs.is_invalid() || rhs.is_invalid() {
                    return Ok(Value::None);
                }

                match Value::binary_op(lhs, rhs, op.as_str()) {
                    Err(err) => {
                        context.error(self, err)?;
                        Ok(Value::None)
                    }
                    Ok(value) => Ok(value),
                }
            }
            Self::UnaryOp {
                op,
                rhs,
                src_ref: _,
            } => {
                let value: Value = rhs.eval(context)?;
                value.unary_op(op.as_str()).map_err(EvalError::ValueError)
            }
            Self::ArrayElementAccess(lhs, rhs, _) => {
                let lhs = lhs.eval(context)?;
                let rhs = rhs.eval(context)?;

                match (lhs, rhs) {
                    (Value::Array(list), Value::Integer(index)) => {
                        let index = index as usize;
                        if index < list.len() {
                            match list.get(index) {
                                Some(value) => Ok(value.clone()),
                                None => Err(EvalError::ListIndexOutOfBounds {
                                    index,
                                    len: list.len(),
                                }),
                            }
                        } else {
                            context.error(
                                self,
                                EvalError::ListIndexOutOfBounds {
                                    index,
                                    len: list.len(),
                                },
                            )?;
                            Ok(Value::None)
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            Self::MethodCall(lhs, method_call, _) => method_call.eval(context, lhs),
            // Access a property `x` of an expression `circle.x`
            Self::PropertyAccess(lhs, id, src_ref) => {
                let value: Value = lhs.eval(context)?;
                match value {
                    Value::Tuple(tuple) => match tuple.by_id(id) {
                        Some(value) => return Ok(value.clone()),
                        None => context.error(src_ref, EvalError::PropertyNotFound(id.clone()))?,
                    },
                    _ => {}
                }

                Ok(Value::None)
            }
            Self::AttributeAccess(lhs, identifier, src_ref) => {
                let value: Value = lhs.eval(context)?;
                let value = value.get_attribute_value(identifier);
                if value == Value::None {
                    context.error(src_ref, AttributeError::NotFound(identifier.clone()))?;
                }
                Ok(value)
            }
            expr => todo!("{expr:?}"),
        };
        match &result {
            Ok(Value::None) => {
                log::warn!(
                    "Expression resulted in invalid value:\n{self}\n--- into ---\n{}",
                    Value::None
                )
            }
            Ok(result) => log::trace!("Evaluated expression:\n{self}\n--- into ---\n{result}"),
            Err(_) => log::trace!("Evaluation of expression failed:\n{self}"),
        };
        result
    }
}
