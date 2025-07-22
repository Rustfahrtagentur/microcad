// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model::*, syntax::*};

impl InitDefinition {
    /// Evaluate a call to the init definition
    pub fn eval(&self, args: Tuple, context: &mut Context) -> EvalResult<()> {
        let model = context.get_model()?;
        context.scope(StackFrame::Init(args.into()), |context| {
            for statement in self.body.statements.iter() {
                match statement {
                    Statement::Assignment(assignment) => {
                        let assignment = &assignment.assignment;
                        let id = &assignment.id;
                        let value: Value = assignment.expression.eval(context)?;

                        // if assignment aims a property set it otherwise set local variable
                        if model.borrow().get_property(id).is_some() {
                            model.borrow_mut().set_property(id.clone(), value.clone());
                        } else {
                            context.set_local_value(id.clone(), value)?;
                        }
                    }
                    _ => {
                        context.error(
                            self,
                            EvalError::StatementNotSupported(statement.clone().into()),
                        )?;
                    }
                }
            }

            Ok(())
        })
    }
}
