// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, objects::*, syntax::*};

impl ModuleInitDefinition {
    /// Evaluate a call to the module init definition
    pub fn eval_to_node(
        &self,
        args: &ArgumentMap,
        object_builder: &mut ObjectBuilder,
        context: &mut Context,
    ) -> EvalResult<()> {
        context.scope(StackFrame::ModuleInit(args.into()), |context| {
            for statement in &self.body.statements {
                match statement {
                    Statement::Assignment(assignment) => {
                        let _id = &assignment.id;
                        let _value = assignment.expression.eval(context)?;
                        todo!();
                    }
                    Statement::Expression(expression) => {
                        object_builder
                            .append_children(&mut expression.eval(context)?.fetch_nodes());
                    }
                    _ => {
                        context.error(self, EvalError::StatementNotSupported(statement.clone()))?;
                    }
                }
            }

            Ok(())
        })
    }
}
