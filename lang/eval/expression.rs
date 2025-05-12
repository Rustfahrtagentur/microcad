// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, objects::*};

impl Eval for ListExpression {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        let mut value_list = ValueList::new(
            self.list
                .iter()
                .map(|expr| expr.eval(context))
                .collect::<Result<_, _>>()?,
        );

        if let Some(unit) = self.unit {
            value_list.add_unit_to_unitless(unit)?;
        }

        match value_list.types().common_type() {
            Some(common_type) => Ok(Value::List(List::new(value_list, common_type))),
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
                let lhs = lhs.eval(context)?;
                let rhs = rhs.eval(context)?;
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
            } => rhs
                .eval(context)?
                .unary_op(op.as_str())
                .map_err(EvalError::ValueError),
            Self::ListElementAccess(lhs, rhs, _) => {
                let lhs = lhs.eval(context)?;
                let rhs = rhs.eval(context)?;

                match (lhs, rhs) {
                    (Value::List(list), Value::Integer(index)) => {
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
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        let mut node_stack = Vec::new();

        for (index, item) in self.iter().enumerate() {
            let value = item.eval(context)?;
            let nodes = match value {
                Value::Node(_) | Value::NodeMultiplicity(_) => value.fetch_nodes(),
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

        Ok(nest_nodes(&node_stack).clone().into())
    }
}

impl Eval for NestedItem {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        match &self {
            NestedItem::Call(call) => Ok(call.eval(context)?),
            NestedItem::QualifiedName(name) => match &context.lookup(name)?.borrow().def {
                SymbolDefinition::Constant(_, value)
                | SymbolDefinition::CallArgument(_, value)
                | SymbolDefinition::Property(_, value) => Ok(value.clone()),
                SymbolDefinition::Namespace(ns) => {
                    Err(EvalError::UnexpectedNested("namespace", ns.id.clone()))
                }
                SymbolDefinition::Module(md) => {
                    Err(EvalError::UnexpectedNested("module", md.id.clone()))
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
            NestedItem::Body(body) => {
                Ok(Value::Node(body.eval_to_node(SymbolMap::new(), context)?))
            }
        }
    }
}
