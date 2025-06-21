// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Workbench definition syntax element evaluation

use crate::{eval::*, model_tree::*, syntax::*};

impl WorkbenchDefinition {
    /// Find a matching initializer for argument list
    fn find_matching_initializer(
        &self,
        args: &ArgumentValueList,
        context: &mut Context,
    ) -> Option<(&InitDefinition, MultiArgumentMap)> {
        self.inits().find_map(|init| {
            if let Ok(arg_map) = args.get_multi_matching_arguments(context, &init.parameters) {
                Some((init, arg_map))
            } else {
                None
            }
        })
    }

    /// Try to evaluate a single call into a [`ModelNode`].
    fn eval_to_node<'a>(
        &'a self,
        arguments: &ArgumentMap,
        init: Option<&'a InitDefinition>,
        context: &mut Context,
    ) -> EvalResult<ModelNode> {
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

                // Create the object node from initializer if present
                if let Some(init) = init {
                    init.eval(arguments, &mut node_builder, context)?;
                }

                node_builder.properties.eval(context)?;

                // At this point, all properties must have a value
                for statement in self.body.statements.iter() {
                    match statement {
                        Statement::Assignment(assignment) => {
                            assignment.eval(context)?;
                        }
                        Statement::Expression(expression) => {
                            let value = expression.eval(context)?;
                            node_builder.add_children(value.fetch_nodes())?;
                        }
                        _ => {}
                    }
                }

                Ok(node_builder
                    .build()
                    .set_original_arguments(arguments.clone())
                    .set_metadata(self.attribute_list.eval(context)?))
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
        let mut nodes = ModelNodes::default();

        match self.find_matching_initializer(args, context) {
            Some((init, multi_args)) => {
                // We have a found a matching initializer. Evaluate each argument combination into a node
                for args in multi_args.combinations() {
                    nodes.push(self.eval_to_node(&args, Some(init), context)?);
                }
            }
            None => match args.get_multi_matching_arguments(context, &self.plan) {
                Ok(multi_args) => {
                    for args in multi_args.combinations() {
                        nodes.push(self.eval_to_node(&args, None, context)?);
                    }
                }
                Err(err) => {
                    context.error(self, err)?;
                }
            },
        }

        Ok(nodes)
    }
}
