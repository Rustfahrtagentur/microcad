// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

impl Eval for ListExpression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        let mut value_list = ValueList::new(Vec::new(), self.src_ref());
        for expr in self.list.clone() {
            value_list.push(expr.eval(context)?);
        }
        if let Some(unit) = self.unit {
            value_list.add_unit_to_unitless(unit)?;
        }

        match value_list.types().common_type() {
            Some(common_type) => Ok(Value::List(List::new(
                value_list,
                common_type,
                self.src_ref(),
            ))),
            None => {
                context.error(self, EvalError::ListElementsDifferentTypes(value_list.types()))?;
                Ok(Value::None)
            }
        }
    }
}

impl Eval for Expression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        match self {
            Self::Literal(literal) => Literal::eval(literal, context),
            Self::FormatString(format_string) => FormatString::eval(format_string, context),
            Self::ListExpression(list_expression) => ListExpression::eval(list_expression, context),
            /*Self::TupleExpression(tuple_expression) => {
                TupleExpression::eval(tuple_expression, context)
            }*/
            Self::BinaryOp {
                lhs,
                op,
                rhs,
                src_ref: _,
            } => {
                let lhs = lhs.eval(context)?;
                let rhs = rhs.eval(context)?;
                if lhs.is_invalid() || rhs.is_invalid() {
                    return Ok(Value::None);
                }

                Value::binary_op(lhs, rhs, op.as_str()).map_err(EvalError::ValueError)
            }
            Self::UnaryOp {
                op,
                rhs,
                src_ref: _,
            } => rhs
                .eval(context)?
                .unary_op(op.as_str())
                .map_err(EvalError::ValueError),
            Self::ListElementAccess(lhs, rhs, _) => {
                let lhs = lhs.eval(context)?;
                let rhs = rhs.eval(context)?;

                match (lhs, rhs) {
                    (Value::List(list), Value::Integer(index)) => {
                        let index = index.value as usize;
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
            Self::Nested(nested) => nested.eval(context),
            Self::PropertyAccess(lhs, identifier, src_ref) => {
                let value = lhs.eval(context)?;

                if let Some(property_value) = value.get_property_value(identifier) {
                    Ok(property_value)
                } else {
                    context.error(src_ref, EvalError::PropertyNotFound(identifier.clone()))?;
                    Ok(Value::None)
                }
            }
            expr => todo!("{expr:?}"),
        }
    }
}

impl Eval for Nested {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        let mut nodes = Vec::new();

        for item in self.iter() {
            match item.eval(context)? {
                Value::Node(n) => nodes.push(n),
                Value::None => {
                    if nodes.is_empty() && self.len() == 1 {
                        return Ok(Value::None);
                    } else {
                        context.error(self, EvalError::CannotNestItem(item.clone()))?;
                    }
                }
                value => {
                    if nodes.is_empty() && self.len() == 1 {
                        return Ok(value);
                    } else {
                        context.error(self, EvalError::CannotNestItem(item.clone()))?;
                    }
                }
            }
        }

        if nodes.is_empty() {
            Ok(Value::None)
        } else {
            todo!("Nest nodes is WIP")
            //Ok(Value::Node(crate::objects::nest_nodes(nodes)))
        }
    }
}

impl Eval for NestedItem {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        match &self {
            NestedItem::Call(call) => Ok(call.eval(context)?),
            NestedItem::QualifiedName(qualified_name) => Ok(qualified_name.eval(context)?),
            NestedItem::Body(body) => Ok(body.eval(context)?),
        }
    }
}
