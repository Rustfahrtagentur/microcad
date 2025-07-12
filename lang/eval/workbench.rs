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
                            node_builder = node_builder.add_children(value.fetch_nodes())?;
                        }
                        _ => {}
                    }
                }

                Ok(node_builder
                    .build()
                    .set_original_arguments(arguments.clone())
                    .set_attributes(self.attribute_list.eval(context)?))
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

        // call initializer
        for init in self.inits() {
            let params = init.parameters.eval(context)?;
            match ArgumentMatch::find_match(args, &params) {
                Ok(args) => {
                    let multipliers = ArgumentMatch::multipliers(&args, &params);
                    for args in args.combinations(multipliers) {
                        nodes.push(self.eval_to_node(&args, Some(init), context)?);
                    }
                    break;
                }
                Err(err) => context.error(self, err)?,
            }
        }

        Ok(nodes)
    }
}
