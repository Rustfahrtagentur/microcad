// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Workbench definition syntax element evaluation

use crate::{eval::*, model_tree::*, syntax::*};

impl WorkbenchDefinition {
    /// Try to evaluate a single call into a [`ModelNode`].
    fn eval_to_node<'a>(
        &'a self,
        arguments: &Tuple,
        init: Option<&'a InitDefinition>,
        context: &mut Context,
    ) -> EvalResult<ModelNode> {
        log::debug!(
            "Evaluating to node `{id}` {kind}",
            id = self.id,
            kind = self.kind
        );
        context.scope(
            StackFrame::Workbench(self.kind, self.id.clone(), arguments.into()),
            |context| {
                let mut node_builder = match self.kind {
                    WorkbenchKind::Part => ModelNodeBuilder::new_3d_object(),
                    WorkbenchKind::Sketch => ModelNodeBuilder::new_2d_object(),
                    WorkbenchKind::Operation => ModelNodeBuilder::new_object_body(),
                }
                .properties(ObjectProperties::from_parameters_and_arguments(
                    &self.plan.eval(context)?,
                    arguments,
                ));
                log::trace!("Prepared node builder:\n{node_builder}");

                // Create the object node from initializer if present
                if let Some(init) = init {
                    log::trace!("Initializing`{id}` {kind}", id = self.id, kind = self.kind);
                    init.eval(arguments, &mut node_builder, context)?;
                }

                node_builder.properties.eval(context)?;

                // At this point, all properties must have a value
                log::trace!("Run body`{id}` {kind}", id = self.id, kind = self.kind);
                for statement in self.body.statements.iter() {
                    match statement {
                        Statement::Assignment(assignment) => {
                            assignment.eval(context)?;
                        }
                        Statement::Expression(expression) => {
                            node_builder = node_builder.add_children(expression.eval(context)?)?;
                        }
                        Statement::Init(_) => (),
                        _ => todo!("Evaluate statement: {statement}"),
                    }
                }

                let node = node_builder.build();
                {
                    let mut node_ = node.borrow_mut();
                    node_.origin.arguments = arguments.clone();
                    node_.attributes = self.attribute_list.eval(context)?;
                }
                Ok(node)
            },
        )
    }
}

impl CallTrait<ModelNodes> for WorkbenchDefinition {
    /// Evaluate the call of a part initialization
    ///
    /// The evaluation considers multiplicity, which means that multiple nodes maybe created.
    ///
    /// Example:
    /// Consider the `part a(b: Scalar) { }`.
    /// Calling the part `a([1.0, 2.0])` results in two nodes with `b = 1.0` and `b = 2.0`, respectively.
    fn call(&self, args: &ArgumentValueList, context: &mut Context) -> EvalResult<ModelNodes> {
        log::debug!(
            "Workbench call `{id}` {kind}",
            id = self.id,
            kind = self.kind
        );

        // prepare nodes
        let mut nodes = ModelNodes::default();

        // prepare building plan
        let plan = self.plan.eval(context)?;

        if let Ok(multi_args) = ArgumentMatch::find_multi_match(args, &plan) {
            for args in multi_args {
                for (id, var) in args.named_iter() {
                    context.set_local_value(id.clone(), var.clone())?;
                }
                nodes.push(self.eval_to_node(&args, None, context)?);
            }
        } else {
            // put all default parameters in the building plan into local variables
            plan.iter().try_for_each(|(id, arg)| {
                if let Some(def) = &arg.default_value {
                    context.set_local_value(id.clone(), def.clone())?;
                }
                EvalResult::Ok(())
            })?;

            for init in self.inits() {
                if let Ok(multi_args) =
                    ArgumentMatch::find_multi_match(args, &init.parameters.eval(context)?)
                {
                    for args in multi_args {
                        nodes.push(self.eval_to_node(&args, Some(init), context)?);
                    }
                    break;
                }
            }
        }

        Ok(nodes)
    }
}
