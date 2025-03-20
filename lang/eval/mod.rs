// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluation of parsed content

mod builtin_function;
mod builtin_module;
mod call;
mod eval_context;
mod eval_error;
mod scope_stack;

pub use builtin_function::*;
pub use builtin_module::*;
pub use call::*;
pub use eval_context::*;
pub use eval_error::*;

use super::*;
use crate::{src_ref::*, r#type::*};
use scope_stack::*;

/// Evaluation trait
pub trait Eval {
    /// Evaluate the type into an expression
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value>;
}

impl Eval for Assignment {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        let value = self.value.eval(context)?;
        context.add_local_value(self.name.id().clone(), value.clone());
        Ok(value)
    }
}

impl Eval for Body {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        Body::evaluate_vec(&self.statements, context)
    }
}

impl Eval for NumberLiteral {
    fn eval(&self, _: &mut EvalContext) -> EvalResult<Value> {
        match self.1.ty() {
            Type::Scalar => Ok(Value::Scalar(Refer::new(
                self.normalized_value(),
                self.src_ref(),
            ))),
            Type::Angle => Ok(Value::Angle(Refer::new(
                self.normalized_value(),
                self.src_ref(),
            ))),
            Type::Length => Ok(Value::Length(Refer::new(
                self.normalized_value(),
                self.src_ref(),
            ))),
            Type::Weight => Ok(Value::Weight(Refer::new(
                self.normalized_value(),
                self.src_ref(),
            ))),
            Type::Area => Ok(Value::Area(Refer::new(
                self.normalized_value(),
                self.src_ref(),
            ))),
            Type::Volume => Ok(Value::Volume(Refer::new(
                self.normalized_value(),
                self.src_ref(),
            ))),
            _ => unreachable!(),
        }
    }
}

impl Eval for Literal {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        match self {
            Literal::Integer(i) => Ok(Value::Integer(i.clone().map(|i| i))),
            Literal::Number(n) => n.eval(context),
            Literal::Bool(b) => Ok(Value::Bool(b.clone().map(|b| b))),
            Literal::Color(c) => Ok(Value::Color(c.clone().map(|c| c))),
        }
    }
}

impl Eval for Nested {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        let mut nodes = Vec::new();

        for item in self.iter() {
            match item.eval(context)? {
                Value::Node(n) => nodes.push(n),
                Value::Invalid => {
                    if nodes.is_empty() && self.len() == 1 {
                        return Ok(Value::Invalid);
                    } else {
                        context.error_with_stack_trace(
                            self,
                            EvalError::CannotNestItem(item.clone()),
                        )?;
                    }
                }
                value => {
                    if nodes.is_empty() && self.len() == 1 {
                        return Ok(value);
                    } else {
                        context.error_with_stack_trace(
                            self,
                            EvalError::CannotNestItem(item.clone()),
                        )?;
                    }
                }
            }
        }

        if nodes.is_empty() {
            Ok(Value::Invalid)
        } else {
            todo!("Nest nodes is WIP")
            //Ok(Value::Node(crate::objects::nest_nodes(nodes)))
        }
    }
}

impl Eval for NestedItem {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        match &self {
            NestedItem::Call(call) => todo!("{call:#?}"),
            NestedItem::QualifiedName(qualified_name) => todo!("{qualified_name:#?}"),
            NestedItem::Body(body) => Ok(body.eval(context)?),
        }
    }
}

impl Eval for Expression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        match self {
            Self::Literal(literal) => Literal::eval(literal, context),
            /*Self::FormatString(format_string) => FormatString::eval(format_string, context),
            Self::ListExpression(list_expression) => ListExpression::eval(list_expression, context),
            Self::TupleExpression(tuple_expression) => {
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
                    return Ok(Value::Invalid);
                }

                match op.as_str() {
                    "+" => lhs + rhs,
                    "-" => lhs - rhs,
                    "*" => lhs * rhs,
                    "/" => lhs / rhs,
                    "^" => unimplemented!(), // lhs.pow(&rhs),
                    "&" => lhs & rhs,
                    "|" => lhs | rhs,
                    ">" => Ok(Value::Bool(Refer::new(lhs > rhs, SrcRef::merge(lhs, rhs)))),
                    "<" => Ok(Value::Bool(Refer::new(lhs < rhs, SrcRef::merge(lhs, rhs)))),
                    "≤" => Ok(Value::Bool(Refer::new(lhs <= rhs, SrcRef::merge(lhs, rhs)))),
                    "≥" => Ok(Value::Bool(Refer::new(lhs >= rhs, SrcRef::merge(lhs, rhs)))),
                    "~" => todo!("implement near ~="),
                    "=" => Ok(Value::Bool(Refer::new(lhs == rhs, SrcRef::merge(lhs, rhs)))),
                    "!=" => Ok(Value::Bool(Refer::new(lhs != rhs, SrcRef::merge(lhs, rhs)))),
                    _ => unimplemented!("{op:?}"),
                }
                .map_err(EvalError::ValueError)
            }
            Self::UnaryOp {
                op,
                rhs,
                src_ref: _,
            } => {
                let rhs = rhs.eval(context)?;
                match op.as_str() {
                    "-" => -rhs.clone(),
                    _ => unimplemented!(),
                }
                .map_err(EvalError::ValueError)
            }
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
                            context.error_with_stack_trace(
                                self,
                                EvalError::ListIndexOutOfBounds {
                                    index,
                                    len: list.len(),
                                },
                            )?;
                            Ok(Value::Invalid)
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            Self::Nested(nested) => nested.eval(context),
            expr => todo!("{expr:#?}"),
        }
    }
}

impl Eval for Statement {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        match self {
            Self::Use(u) => u.eval(context)?,
            Self::Assignment(a) => a.eval(context)?,
            Self::Expression(e) => e.eval(context)?,
            statement => todo!("{statement:#?}"),
        };

        Ok(Value::Invalid)
    }
}

impl Eval for UseStatement {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        for decl in &self.decls {
            decl.eval(context)?;
        }
        Ok(Value::Invalid)
    }
}

impl Eval for UseDeclaration {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        match &self {
            UseDeclaration::Use(qualified_name, src_ref) => context.use_symbol(qualified_name),
            UseDeclaration::UseAll(qualified_name, src_ref) => todo!(),
            UseDeclaration::UseAlias(qualified_name, identifier, src_ref) => todo!(),
        };
        Ok(Value::Invalid)
    }
}
