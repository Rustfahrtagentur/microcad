// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model::*};

impl Eval for RangeStart {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        let value: Value = self.0.eval(context)?;
        Ok(match value {
            Value::Integer(_) => value,
            value => {
                context.error(
                    self,
                    EvalError::ExpectedType {
                        expected: Type::Integer,
                        found: value.ty(),
                    },
                )?;

                Value::None
            }
        })
    }
}

impl Eval for RangeEnd {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        let value: Value = self.0.eval(context)?;
        Ok(match value {
            Value::Integer(_) => value,
            value => {
                context.error(
                    self,
                    EvalError::ExpectedType {
                        expected: Type::Integer,
                        found: value.ty(),
                    },
                )?;

                Value::None
            }
        })
    }
}

impl Eval for RangeExpression {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        Ok(match (self.start.eval(context)?, self.end.eval(context)?) {
            (Value::Integer(start), Value::Integer(end)) => Value::Array(Array::new(
                (start..end).map(Value::Integer).collect(),
                Type::Integer,
            )),
            (_, _) => Value::None,
        })
    }
}

impl Eval for ArrayExpression {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        match &self.inner {
            ArrayExpressionInner::Range(range_expression) => range_expression.eval(context),
            ArrayExpressionInner::List(expressions) => {
                let value_list = ValueList::new(
                    expressions
                        .iter()
                        .map(|expr| expr.eval(context))
                        .collect::<Result<_, _>>()?,
                );

                match value_list.types().common_type() {
                    Some(common_type) => {
                        match Value::Array(Array::new(value_list, common_type)) * self.unit {
                            Ok(value) => Ok(value),
                            Err(err) => {
                                context.error(self, err)?;
                                Ok(Value::None)
                            }
                        }
                    }
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
            Value::Models(mut models) => {
                let attributes = attribute_list.eval(context)?;
                models.iter_mut().for_each(|model| {
                    model.borrow_mut().attributes = attributes.clone();
                });
                Ok(Value::Models(models))
            }
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
            Self::Nested(nested) => nested.eval(context),
            Self::Marker(marker) => {
                let model: Option<Model> = marker.eval(context)?;
                Ok(Value::Models(model.into_iter().collect()))
            }
            // Access a property `x` of an expression `circle.x`
            Self::PropertyAccess(lhs, id, src_ref) => {
                let value: Value = lhs.eval(context)?;
                match value {
                    Value::Tuple(tuple) => match tuple.by_id(id) {
                        Some(value) => return Ok(value.clone()),
                        None => context.error(src_ref, EvalError::PropertyNotFound(id.clone()))?,
                    },
                    Value::Models(models) => match models.fetch_property(id) {
                        Some(prop) => return Ok(prop),
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

impl Eval<Models> for Expression {
    fn eval(&self, context: &mut Context) -> EvalResult<Models> {
        let value: Value = self.eval(context)?;
        Ok(value.fetch_models())
    }
}

impl Eval for Nested {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        let mut model_stack = Vec::new();

        for (index, item) in self.iter().enumerate() {
            let value = item.eval(context)?;
            let models = match value {
                Value::Models(models) => models,
                Value::None => return Ok(Value::None),
                value => {
                    if index == 0 && self.len() == 1 {
                        return Ok(value);
                    } else {
                        context.error(item, EvalError::CannotNestItem(item.clone()))?;
                        break;
                    }
                }
            };
            model_stack.push(models);
        }

        Ok(Value::Models(Models::from_nested_items(&model_stack)))
    }
}

impl Eval for NestedItem {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        match &self {
            NestedItem::Call(call) => Ok(call.eval(context)?),
            NestedItem::QualifiedName(name) => match &context.lookup(name)?.borrow().def {
                SymbolDefinition::Constant(_, value) | SymbolDefinition::Argument(_, value) => {
                    Ok(value.clone())
                }
                SymbolDefinition::Module(ns) => {
                    Err(EvalError::UnexpectedNested("mod", ns.id.clone()))
                }
                SymbolDefinition::Workbench(w) => {
                    Err(EvalError::UnexpectedNested(w.kind.as_str(), w.id.clone()))
                }
                SymbolDefinition::Function(f) => {
                    Err(EvalError::UnexpectedNested("function", f.id.clone()))
                }
                SymbolDefinition::Builtin(bm) => {
                    Err(EvalError::UnexpectedNested("builtin", bm.id.clone()))
                }
                SymbolDefinition::Alias(id, _) => {
                    unreachable!("Unexpected alias {id} in expression")
                }
                SymbolDefinition::SourceFile(sf) => {
                    unreachable!(
                        "Unexpected source file {} in expression",
                        sf.filename_as_str()
                    )
                }
                SymbolDefinition::External(ns) => {
                    unreachable!("Unexpected unload source file {} in expression", ns.id)
                }
            },
            NestedItem::Body(body) => Ok(Value::from_single_model(body.eval(context)?)),
        }
    }
}
