// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{GetPropertyValue, eval::*, model_tree::*};

impl Eval for ListExpression {
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

impl Eval for Expression {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        match self {
            Self::Literal(literal) => literal.eval(context),
            Self::FormatString(format_string) => format_string.eval(context),
            Self::ListExpression(list_expression) => list_expression.eval(context),
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
            Self::PropertyAccess(lhs, identifier, src_ref) => {
                let value: Value = lhs.eval(context)?;
                let value = value.get_property_value(identifier);
                if value == Value::None {
                    context.error(src_ref, EvalError::PropertyNotFound(identifier.clone()))?;
                }
                Ok(value)
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
        }
    }
}

impl Eval<ModelNodes> for Expression {
    fn eval(&self, context: &mut Context) -> EvalResult<ModelNodes> {
        let value: Value = self.eval(context)?;
        Ok(value.fetch_nodes())
    }
}

impl Eval for Nested {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        let mut node_stack = Vec::new();

        for (index, item) in self.iter().enumerate() {
            let value = item.eval(context)?;
            let nodes = match value {
                Value::Nodes(nodes) => nodes,
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
            node_stack.push(nodes);
        }

        Ok(Value::Nodes(ModelNodes::from_node_stack(&node_stack)))
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
            NestedItem::Body(body) => Ok(Value::from_single_node(body.eval(context)?)),
        }
    }
}
