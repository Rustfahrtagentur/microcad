// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;
use crate::value::*;

impl Assignment {
    /// Check if the specified type matches the found type.
    pub fn type_check(&self, found: Type) -> EvalResult<()> {
        if let Some(ty) = &self.specified_type {
            if ty.ty() != found {
                return Err(EvalError::TypeMismatch {
                    id: self.id.clone(),
                    expected: ty.ty(),
                    found,
                });
            }
        }

        Ok(())
    }
}

impl Eval<()> for Assignment {
    fn eval(&self, context: &mut Context) -> EvalResult<()> {
        let value: Value = self.expression.eval(context)?;

        if let Err(err) = self.type_check(value.ty()) {
            context.error(self, err)?;
            return Ok(());
        }

        context.set_local_value(self.id.clone(), value)?;

        Ok(())
    }
}

impl Eval<()> for AssignmentStatement {
    fn eval(&self, context: &mut Context) -> EvalResult<()> {
        let assignment = &self.assignment;
        let value: Value = assignment.expression.eval(context)?;
        if let Err(err) = assignment.type_check(value.ty()) {
            context.error(self, err)?;
            return Ok(());
        }

        let value = match value {
            Value::Models(mut models) => {
                let attributes = self.attribute_list.eval(context)?;
                models.iter_mut().for_each(|model| {
                    model.borrow_mut().attributes = attributes.clone();
                });
                Value::Models(models)
            }
            value => {
                if !self.attribute_list.is_empty() {
                    context.error(
                        &self.attribute_list,
                        AttributeError::CannotAssignToExpression(
                            self.assignment.expression.clone().into(),
                        ),
                    )?;
                }
                value
            }
        };

        context.set_local_value(assignment.id.clone(), value)?;

        Ok(())
    }
}
