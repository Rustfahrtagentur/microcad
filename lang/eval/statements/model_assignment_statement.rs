// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;
use crate::value::*;

impl Eval<()> for ModelAssignment {
    fn eval(&self, context: &mut Context) -> EvalResult<()> {
        let value: Value = self.expression.eval(context)?;

        if let Err(err) = context.set_local_value(self.id.clone(), value) {
            context.error(self, err)?;
            return Ok(());
        }

        Ok(())
    }
}

impl Eval<()> for ModelAssignmentStatement {
    fn eval(&self, context: &mut Context) -> EvalResult<()> {
        log::debug!("Evaluating model assignment statement:\n{self}");
        // TODO: context.grant(self)?;

        let assignment = &self.assignment;

        let value: Value = assignment.expression.eval(context)?;

        let value = match value {
            Value::Models(mut models) => {
                let attributes = self.attribute_list.eval(context)?;
                models.iter_mut().for_each(|model| {
                    model.borrow_mut().attributes = attributes.clone();
                });
                Value::Models(models)
            }
            _ => unreachable!("Value must be Models"),
        };

        if let Err(err) = context.set_local_value(assignment.id.clone(), value) {
            context.error(self, err)?;
        }

        Ok(())
    }
}
