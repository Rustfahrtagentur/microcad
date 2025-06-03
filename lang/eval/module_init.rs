// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, objects::*, syntax::*};

impl ModuleInitDefinition {
    /// Evaluate a call to the module init definition
    pub fn eval(
        &self,
        args: &ArgumentMap,
        object_builder: &mut ObjectBuilder,
        context: &mut Context,
    ) -> EvalResult<()> {
        context.scope(StackFrame::ModuleInit(args.into()), |context| {
            for statement in self.body.statements.iter() {
                match statement {
                    Statement::Assignment(assignment) => {
                        let assignment = &assignment.assignment;
                        let id = &assignment.id;
                        let value = assignment.expression.eval(context)?;

                        // Only change the property value, do not add new properties
                        if object_builder.has_property(id) {
                            object_builder.set_property(id.clone(), value.clone());
                        }
                        context.set_local_value(id.clone(), value)?;
                    }
                    Statement::Expression(expression) => {
                        object_builder
                            .append_children(&mut expression.eval(context)?.fetch_nodes());
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
