// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model::*};

impl Eval for RangeFirst {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
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

impl Eval for RangeLast {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
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
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        Ok(
            match (self.first.eval(context)?, self.last.eval(context)?) {
                (Value::Integer(first), Value::Integer(last)) => Value::Array(Array::new(
                    (first..last + 1).map(Value::Integer).collect(),
                    Type::Integer,
                )),
                (_, _) => Value::None,
            },
        )
    }
}

impl Eval for ArrayExpression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
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
                            EvalError::ArrayElementsDifferentTypes(value_list.types()),
                        )?;
                        Ok(Value::None)
                    }
                }
            }
        }
    }
}

impl Eval<Option<Symbol>> for QualifiedName {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Option<Symbol>> {
        match context.lookup(self) {
            Ok(symbol) => Ok(Some(symbol.clone())),
            Err(error) => {
                context.error(self, error)?;
                Ok(None)
            }
        }
    }
}

impl Eval for QualifiedName {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        context.lookup(self)?.with_def(|def| match def {
            SymbolDefinition::Constant(.., value) | SymbolDefinition::Argument(_, value) => {
                Ok(value.clone())
            }
            SymbolDefinition::ConstExpression(.., expr) => expr.eval(context),
            SymbolDefinition::SourceFile(_) => Ok(Value::None),

            SymbolDefinition::Module(ns) => Err(EvalError::UnexpectedNested("mod", ns.id.clone())),
            SymbolDefinition::Workbench(w) => {
                Err(EvalError::UnexpectedNested(w.kind.as_str(), w.id.clone()))
            }
            SymbolDefinition::Function(f) => {
                Err(EvalError::UnexpectedNested("function", f.id.clone()))
            }
            SymbolDefinition::Builtin(bm) => {
                Err(EvalError::UnexpectedNested("builtin", bm.id.clone()))
            }
            SymbolDefinition::Alias(_, id, _) => {
                // Alias should have been resolved within previous lookup()
                unreachable!(
                    "Unexpected alias {id} in value expression at {}",
                    self.src_ref()
                )
            }
            SymbolDefinition::UseAll(_, name) => {
                unreachable!("Unexpected use {name} in value expression")
            }
            #[cfg(test)]
            SymbolDefinition::Tester(..) => {
                unreachable!()
            }
        })
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
        context: &mut EvalContext,
    ) -> EvalResult<Value> {
        let value = self.eval(context)?;
        match value {
            Value::Model(model) => {
                let attributes = attribute_list.eval(context)?;
                model.borrow_mut().attributes = attributes.clone();
                Ok(Value::Model(model))
            }
            Value::None => Ok(Value::None),
            _ => {
                if !attribute_list.is_empty() {
                    context.error(
                        attribute_list,
                        AttributeError::CannotAssignAttribute(self.to_string()),
                    )?;
                }
                Ok(value)
            }
        }
    }
}

impl Eval for Expression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
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
            Self::Call(call) => call.eval(context),
            Self::Body(body) => {
                let model: Model = body.eval(context)?;
                Ok(model.into())
            }
            Self::QualifiedName(qualified_name) => qualified_name.eval(context),
            Self::Marker(marker) => {
                let model: Option<Model> = marker.eval(context)?;
                Ok(model.map(Value::Model).unwrap_or_default())
            }
            // Access a property `x` of an expression `circle.x`
            Self::PropertyAccess(lhs, id, src_ref) => {
                let value: Value = lhs.eval(context)?;
                match value {
                    Value::Tuple(tuple) => match tuple.by_id(id) {
                        Some(value) => return Ok(value.clone()),
                        None => context.error(src_ref, EvalError::PropertyNotFound(id.clone()))?,
                    },
                    Value::Model(model) => match model.borrow().get_property(id) {
                        Some(prop) => return Ok(prop.clone()),
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
            Ok(result) => log::trace!("Evaluated expression:\n{self:?}\n--- into ---\n{result:?}"),
            Err(_) => log::trace!("Evaluation of expression failed:\n{self:?}"),
        };
        result
    }
}

impl Eval<Option<Model>> for Expression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Option<Model>> {
        Ok(match self.eval(context)? {
            Value::Model(model) => Some(model),
            _ => None,
        })
    }
}
