// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Workbench definition syntax element evaluation
// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model_tree::*};

mod assignment;
mod r#if;

impl Eval for ExpressionStatement {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        let value: Value = self.expression.eval(context)?;
        match value {
            Value::Nodes(mut nodes) => {
                let attributes = self.attribute_list.eval(context)?;
                nodes.iter_mut().for_each(|node| {
                    node.borrow_mut().attributes = attributes.clone();
                });
                Ok(Value::Nodes(nodes))
            }
            Value::None => Ok(Value::None),
            _ => {
                if !self.attribute_list.is_empty() {
                    context.error(
                        &self.attribute_list,
                        AttributeError::CannotAssignToExpression(self.expression.clone().into()),
                    )?;
                }
                Ok(value)
            }
        }
    }
}

impl Eval<ModelNodes> for ExpressionStatement {
    fn eval(&self, context: &mut Context) -> EvalResult<ModelNodes> {
        let value: Value = self.eval(context)?;
        Ok(value.fetch_nodes())
    }
}

impl Eval<Option<ModelNode>> for Marker {
    fn eval(&self, _: &mut Context) -> EvalResult<Option<ModelNode>> {
        if self.is_children_marker() {
            Ok(Some(ModelNodeBuilder::new_children_placeholder().build()))
        } else {
            Ok(None)
        }
    }
}

impl Eval<ModelNodes> for Marker {
    fn eval(&self, context: &mut Context) -> EvalResult<ModelNodes> {
        let node: Option<ModelNode> = self.eval(context)?;
        Ok(node.into())
    }
}

impl Eval for Statement {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        Ok(match self {
            Self::Use(u) => u.eval(context)?,
            Self::Assignment(a) => {
                a.eval(context)?;
                Value::None
            }
            Self::If(i) => i.eval(context)?,
            Self::Expression(e) => e.eval(context)?,
            Self::Workbench(_) | Self::Function(_) | Self::Module(_) | Self::Marker(_) => {
                Value::None
            }
            statement => todo!("{statement}"),
        })
    }
}

impl Eval<ModelNodes> for Statement {
    fn eval(&self, context: &mut Context) -> EvalResult<ModelNodes> {
        let nodes: ModelNodes = match self {
            Self::Use(u) => {
                u.eval(context)?;
                ModelNodes::default()
            }
            Self::Assignment(a) => {
                a.eval(context)?;
                ModelNodes::default()
            }
            Self::If(i) => {
                let node: Option<ModelNode> = i.eval(context)?;
                node.into()
            }
            Self::Expression(e) => e.eval(context)?,
            _ => ModelNodes::default(),
        };

        if nodes.deduce_output_type() == ModelNodeOutputType::InvalidMixed {
            context.error(self, EvalError::CannotMixGeometry)?;
        }
        Ok(nodes)
    }
}

impl Eval<ModelNodes> for StatementList {
    fn eval(&self, context: &mut Context) -> EvalResult<ModelNodes> {
        let mut nodes = ModelNodes::default();
        let mut output_type = ModelNodeOutputType::NotDetermined;

        for statement in self.iter() {
            let mut statement_nodes: ModelNodes = statement.eval(context)?;
            output_type = output_type.merge(&statement_nodes.deduce_output_type());
            if output_type == ModelNodeOutputType::InvalidMixed {
                context.error(statement, EvalError::CannotMixGeometry)?;
            }

            nodes.append(&mut statement_nodes);
        }

        Ok(nodes)
    }
}
