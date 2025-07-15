// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model_tree::*};

/// Evaluate the body into a collection of model nodes.
impl Eval<ModelNodes> for Body {
    fn eval(&self, context: &mut Context) -> EvalResult<ModelNodes> {
        self.statements
            .iter()
            .map(|statement| -> EvalResult<ModelNodes> {
                match statement {
                    Statement::Use(_) => {} // Use statements have been resolved at this point
                    Statement::Assignment(assignment) => {
                        assignment.eval(context)?;
                    }
                    Statement::Expression(expression) => return expression.eval(context),
                    Statement::Marker(marker) => return marker.eval(context),
                    Statement::If(r#if) => {
                        let node: Option<ModelNode> = r#if.eval(context)?;
                        return Ok(node.into());
                    }
                    statement => {
                        use crate::diag::PushDiag;
                        context.error(
                            self,
                            EvalError::StatementNotSupported(statement.clone().into()),
                        )?;
                    }
                }

                Ok(ModelNodes::default())
            })
            .try_fold(ModelNodes::default(), |mut model_nodes, new| match new {
                Ok(mut new_nodes) => {
                    model_nodes.append(&mut new_nodes);
                    model_nodes.deduce_output_type();
                    Ok(model_nodes)
                }
                _ => todo!(),
            })
    }
}

/// Evaluate the body into a single object body node: `{}`.
impl Eval<ModelNode> for Body {
    fn eval(&self, context: &mut Context) -> EvalResult<ModelNode> {
        context.scope(StackFrame::Body(SymbolMap::default()), |context| {
            Ok(ModelNodeBuilder::new_object_body()
                .add_children(self.eval(context)?)?
                .build())
        })
    }
}
