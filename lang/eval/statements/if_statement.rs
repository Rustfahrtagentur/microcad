// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! If statement evaluation.

use crate::{eval::*, model::*, syntax::*, value::*};

impl Eval<Value> for IfStatement {
    fn eval(&self, context: &mut crate::eval::Context) -> crate::eval::EvalResult<Value> {
        context.grant(self)?;
        todo!("evaluate if statement in function")
    }
}

impl Eval<Option<Model>> for IfStatement {
    fn eval(&self, context: &mut Context) -> EvalResult<Option<Model>> {
        context.grant(self)?;
        let cond = self.cond.eval(context)?;
        match cond {
            Value::Bool(true) => Ok(Some(self.body.eval(context)?)),
            Value::Bool(false) => {
                if let Some(body) = &self.body_else {
                    Ok(Some(body.eval(context)?))
                } else {
                    Ok(None)
                }
            }
            _ => {
                context.error(self, EvalError::IfConditionIsNotBool(cond.clone()))?;
                Ok(None)
            }
        }
    }
}
