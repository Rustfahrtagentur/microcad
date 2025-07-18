// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model_tree::*};

/// Evaluate the body into a collection of models.
impl Eval<Models> for Body {
    fn eval(&self, context: &mut Context) -> EvalResult<Models> {
        self.statements
            .iter()
            .map(|statement| -> EvalResult<Models> {
                match statement {
                    Statement::Use(_) => {} // Use statements have been resolved at this point
                    Statement::Assignment(assignment) => {
                        assignment.eval(context)?;
                    }
                    Statement::Expression(expression) => return expression.eval(context),
                    Statement::Marker(marker) => return marker.eval(context),
                    Statement::If(r#if) => {
                        let model: Option<Model> = r#if.eval(context)?;
                        return Ok(model.into());
                    }
                    statement => {
                        use crate::diag::PushDiag;
                        context.error(
                            self,
                            EvalError::StatementNotSupported(statement.clone().into()),
                        )?;
                    }
                }

                Ok(Models::default())
            })
            .try_fold(Models::default(), |mut models, new| match new {
                Ok(mut new_models) => {
                    models.append(&mut new_models);
                    models.deduce_output_type();
                    Ok(models)
                }
                Err(err) => Err(err),
            })
    }
}

/// Evaluate the body into a single object body model: `{}`.
impl Eval<Model> for Body {
    fn eval(&self, context: &mut Context) -> EvalResult<Model> {
        context.scope(StackFrame::Body(SymbolMap::default()), |context| {
            Ok(ModelBuilder::new_object_body()
                .add_children(self.eval(context)?)?
                .build())
        })
    }
}
