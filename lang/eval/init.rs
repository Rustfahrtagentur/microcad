// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, syntax::*};

impl InitDefinition {
    /// Evaluate a call to the init definition
    pub fn eval(&self, args: &Tuple, context: &mut Context) -> EvalResult<()> {
        let model_builder = context.get_model_builder()?;
        let mut model_builder = model_builder.borrow_mut();

        context.scope(StackFrame::Init(args.into()), |context| {
            for statement in self.body.statements.iter() {
                match statement {
                    Statement::Assignment(assignment) => {
                        let assignment = &assignment.assignment;
                        let id = &assignment.id;
                        let value: Value = assignment.expression.eval(context)?;

                        // Only change the property value, do not add new properties
                        if model_builder.has_property(id) {
                            model_builder.set_property(id.clone(), value.clone());
                        }
                        context.set_local_value(id.clone(), value)?;
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
