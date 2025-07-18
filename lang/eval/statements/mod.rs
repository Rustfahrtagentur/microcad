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
            Value::Models(mut models) => {
                let attributes = self.attribute_list.eval(context)?;
                models.iter_mut().for_each(|model| {
                    model.borrow_mut().attributes = attributes.clone();
                });
                Ok(Value::Models(models))
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

impl Eval<Models> for ExpressionStatement {
    fn eval(&self, context: &mut Context) -> EvalResult<Models> {
        let value: Value = self.eval(context)?;
        Ok(value.fetch_models())
    }
}

impl Eval<Option<Model>> for Marker {
    fn eval(&self, _: &mut Context) -> EvalResult<Option<Model>> {
        if self.is_children_marker() {
            Ok(Some(ModelBuilder::new_children_placeholder().build()))
        } else {
            Ok(None)
        }
    }
}

impl Eval<Models> for Marker {
    fn eval(&self, context: &mut Context) -> EvalResult<Models> {
        let model: Option<Model> = self.eval(context)?;
        Ok(model.into())
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

impl Eval<Models> for Statement {
    fn eval(&self, context: &mut Context) -> EvalResult<Models> {
        let models: Models = match self {
            Self::Use(u) => {
                u.eval(context)?;
                Models::default()
            }
            Self::Assignment(a) => {
                a.eval(context)?;
                Models::default()
            }
            Self::If(i) => {
                let model: Option<Model> = i.eval(context)?;
                model.into()
            }
            Self::Expression(e) => e.eval(context)?,
            _ => Models::default(),
        };

        if models.deduce_output_type() == OutputType::InvalidMixed {
            context.error(self, EvalError::CannotMixGeometry)?;
        }
        Ok(models)
    }
}

impl Eval<Models> for StatementList {
    fn eval(&self, context: &mut Context) -> EvalResult<Models> {
        let mut models = Models::default();
        let mut output_type = OutputType::NotDetermined;

        for statement in self.iter() {
            let mut statement_models: Models = statement.eval(context)?;
            output_type = output_type.merge(&statement_models.deduce_output_type());
            if output_type == OutputType::InvalidMixed {
                context.error(statement, EvalError::CannotMixGeometry)?;
            }

            models.append(&mut statement_models);
        }

        Ok(models)
    }
}
