// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, objects::*, syntax::*};

impl ModuleInitDefinition {
    /// Evaluate a call to the module init definition
    pub fn eval_to_node(
        &self,
        args: &ArgumentMap,
        mut props: ObjectProperties,
        context: &mut Context,
    ) -> EvalResult<ObjectNode> {
        context.open_body();

        // Add values from argument map as local values
        for (id, value) in args.iter() {
            props.assign_and_add_local_value(id, value.clone(), context)?;
        }

        let mut nodes = Vec::new();
        for statement in &self.body.statements {
            match statement {
                Statement::Assignment(assignment) => {
                    let id = &assignment.id;
                    let value = assignment.expression.eval(context)?;

                    props.assign_and_add_local_value(id, value, context)?;
                }
                Statement::Expression(expression) => {
                    nodes.append(&mut expression.eval(context)?.fetch_nodes())
                }
                _ => {
                    context.error(self, EvalError::StatementNotSupported(statement.clone()))?;
                }
            }
        }

        context.close();

        if !props.all_initialized() {
            context.error(
                self,
                EvalError::UninitializedProperties(props.get_ids_of_uninitialized()),
            )?;
            return Ok(empty_object());
        }

        // Make a new object node
        let object = object(Object {
            id: Identifier::none(),
            props,
        });
        for node in nodes {
            object.append(node);
        }
        Ok(object)
    }
}
