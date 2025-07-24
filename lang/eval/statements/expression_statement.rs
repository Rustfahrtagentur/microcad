// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model::*};

impl Eval for ExpressionStatement {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        log::debug!("Evaluating expression statement to value:\n{self}");
        context.grant(self)?;
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
        log::debug!("Evaluating expression statement to models:\n{self}");
        context.grant(self)?;
        let value: Value = self.eval(context)?;
        Ok(value.fetch_models())
    }
}
