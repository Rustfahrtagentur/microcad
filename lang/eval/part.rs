// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Part definition syntax element evaluation

use crate::{eval::*, model_tree::*, rc::*, syntax::*};

impl PartDefinition {
    /// Find a matching initializer for call argument list
    fn find_matching_initializer(
        &self,
        args: &CallArgumentValueList,
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

    /// Resolve into SymbolNode.
    pub fn resolve(self: &Rc<Self>, parent: Option<Symbol>) -> Symbol {
        let node = Symbol::new(SymbolDefinition::Part(self.clone()), parent);
        node.borrow_mut().children = self.body.resolve(Some(node.clone()));
        node
    }

    /// Try to evaluate a single call into a [`ModelNode`].
    fn eval_to_node<'a>(
        &'a self,
        args: &ArgumentMap,
        init: Option<&'a InitDefinition>,
        context: &mut Context,
    ) -> EvalResult<ModelNode> {
        let mut object_builder = ObjectBuilder::default();

        context.scope(StackFrame::Part(self.id.clone(), args.into()), |context| {
            object_builder.init_properties(&self.parameters.eval(context)?, args);
            object_builder.set_metadata(self.attribute_list.eval(context)?);

            // Create the object node from initializer if present
            if let Some(init) = init {
                init.eval(args, &mut object_builder, context)?;
            }

            object_builder.properties_to_scope(context)?;

            // At this point, all properties must have a value
            for statement in self.body.statements.iter() {
                match statement {
                    Statement::Assignment(assignment) => {
                        assignment.eval(context)?;
                    }
                    Statement::Expression(expression) => {
                        let value = expression.eval(context)?;
                        object_builder.append_children(&mut value.fetch_nodes());
                    }
                    _ => {}
                }
            }

            Ok(object_builder.build_node())
        })
    }
}

impl CallTrait for PartDefinition {
    /// Evaluate the call of a part initialization
    ///
    /// The evaluation considers multiplicity, which means that multiple nodes maybe created.
    ///
    /// Example:
    /// Consider the `part a(b: Scalar) { }`.
    /// Calling the part `a([1.0, 2.0])` results in two nodes with `b = 1.0` and `b = 2.0`, respectively.
    fn call(&self, args: &CallArgumentValueList, context: &mut Context) -> EvalResult<Value> {
        let mut nodes = ModelNodes::default();

        match self.find_matching_initializer(args, context) {
            Some((init, multi_args)) => {
                // We have a found a matching initializer. Evaluate each argument combination into a node
                for args in multi_args.combinations() {
                    nodes.push(self.eval_to_node(&args, Some(init), context)?);
                }
            }
            None => match args.get_multi_matching_arguments(context, &self.parameters) {
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

        Ok(nodes.into())
    }
}
