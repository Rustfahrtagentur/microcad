// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model::*};

impl Eval<Models> for ModelExpressionStatement {
    fn eval(&self, context: &mut Context) -> EvalResult<Models> {
        log::debug!("Evaluating model expression statement:\n{self}");
        //todo!("context.grant(self)?");
        let attributes = self.attribute_list.eval(context)?;
        let mut models = self.expression.eval(context)?;
        models.iter_mut().for_each(|model| {
            model.borrow_mut().attributes = attributes.clone();
        });
        if models.is_empty() {
            context.warning(self, EvalError::EmptyModelExpression)?;
        }
        Ok(models)
    }
}
