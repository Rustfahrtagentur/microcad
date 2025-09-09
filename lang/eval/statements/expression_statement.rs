// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model::*};

impl Eval for ExpressionStatement {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        log::debug!("Evaluating expression statement to value:\n{self}");
        context.grant(self)?;
        let value: Value = self.expression.eval(context)?;
        match value {
            Value::Model(model) => {
                let attributes = self.attribute_list.eval(context)?;
                model
                    .borrow_mut()
                    .attributes
                    .append(&mut attributes.clone());
                Ok(Value::Model(model))
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

impl Eval<Option<Model>> for ExpressionStatement {
    fn eval(&self, context: &mut Context) -> EvalResult<Option<Model>> {
        log::debug!("Evaluating expression statement to models:\n{self}");
        Ok(match self.eval(context)? {
            Value::Model(model) => Some(model),
            _ => None,
        })
    }
}
