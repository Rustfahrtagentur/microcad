// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! If statement evaluation.

use crate::{eval::*, model::*, syntax::*, value::*};

impl Eval<Value> for IfStatement {
    fn eval(&self, context: &mut crate::eval::EvalContext) -> crate::eval::EvalResult<Value> {
        log::debug!("Evaluating if statement to value: {self}");
        context.grant(self)?;
        let cond = self.cond.eval(context)?;
        match cond {
            Value::Bool(true) => Ok(self.body.eval(context)?),
            Value::Bool(false) => {
                if let Some(body) = &self.body_else {
                    Ok(body.eval(context)?)
                } else {
                    Ok(Value::None)
                }
            }
            _ => {
                context.error(self, EvalError::IfConditionIsNotBool(cond.to_string()))?;
                Ok(Value::None)
            }
        }
    }
}

impl Eval<Option<Model>> for IfStatement {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Option<Model>> {
        log::debug!("Evaluating if statement to model: {self}");
        context.grant(self)?;
        let cond = self.cond.eval(context)?;
        match cond {
            Value::Bool(true) => Ok(self.body.eval(context)?),
            Value::Bool(false) => {
                if let Some(body) = &self.body_else {
                    Ok(body.eval(context)?)
                } else {
                    Ok(None)
                }
            }
            _ => {
                context.error(self, EvalError::IfConditionIsNotBool(cond.to_string()))?;
                Ok(None)
            }
        }
    }
}
