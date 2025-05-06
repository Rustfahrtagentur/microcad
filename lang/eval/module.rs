// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module definition evaluation
use std::rc::Rc;

use crate::{eval::*, objects::*};

impl ModuleDefinition {
    /// Find a matching initializer for call argument list
    fn find_matching_initializer(
        &self,
        args: &CallArgumentValueList,
        context: &mut Context,
    ) -> Option<(&ModuleInitDefinition, MultiArgumentMap)> {
        self.inits().find_map(|init| {
            if let Ok(arg_map) = args.get_multi_matching_arguments(context, &init.parameters) {
                Some((init, arg_map))
            } else {
                None
            }
        })
    }

    /// Resolve into SymbolNode
    pub fn resolve(self: &Rc<Self>, parent: Option<Symbol>) -> Symbol {
        let node = Symbol::new(SymbolDefinition::Module(self.clone()), parent);
        node.borrow_mut().children = self.body.resolve(Some(node.clone()));
        node
    }

    /// Try to evaluate a single call to an object
    fn eval_to_node<'a>(
        &'a self,
        args: &ArgumentMap,
        init: Option<&'a ModuleInitDefinition>,
        context: &mut Context,
    ) -> EvalResult<ObjectNode> {
        let symbol_map = self.parameters.try_to_symbol_map(context)?;
        context.open_module(self.id.clone(), symbol_map)?;

        let mut object_builder = ObjectBuilder::new();

        // Create the object node from initializer if present
        match init {
            Some(init) => init.eval_to_node(args, context)?,

            // No matching initializer has been found
            None => {
                // Add values from argument map as local values
                for (id, value) in args.iter() {
                    match context.get_local(id) {
                        Ok(symbol) => match &symbol.borrow().def {
                            // Overwrite the property value if the local already exists
                            SymbolDefinition::Property(_, value) => {
                                context.set_local(
                                    id,
                                    Symbol::new_property(id.clone(), value.clone()),
                                )?;
                            }
                            _ => {}
                        },
                        Err(_) => {
                            context.set_local(
                                id,
                                Symbol::new_call_argument(id.clone(), value.clone()),
                            )?;
                        }
                    }
                }

                let props: ObjectProperties = context.top().symbol_map().into();

                if !props.all_initialized() {
                    context.error(
                        self,
                        EvalError::UninitializedProperties(props.get_ids_of_uninitialized()),
                    )?;
                    return Ok(empty_object());
                }

                object(Object {
                    props,
                    ..Default::default()
                })
            }
        };

        // At this point, all properties must have a value

        for statement in &self.body.statements {
            match statement {
                Statement::Assignment(assignment) => {
                    let id = &assignment.id;
                    let value = assignment.expression.eval(context)?;
                    context.set_local_value(id.clone(), value)?;
                }
                Statement::Expression(expression) => {
                    let value = expression.eval(context)?;
                    object_builder.append_children(&mut value.fetch_nodes());
                }
                _ => {}
            }
        }

        Ok(object_builder.build_node())
    }

    /// Evaluate the call of a module
    ///
    /// The evaluation considers multiplicity, which means that multiple nodes may be created.
    ///
    /// Example:
    /// Consider the `module a(b: Scalar) { }`.
    /// Calling the module `a([1.0, 2.0])` results in two nodes with `b = 1.0` and `b = 2.0`, respectively.
    pub fn eval_call(
        &self,
        args: &CallArgumentValueList,
        context: &mut Context,
    ) -> EvalResult<Value> {
        let mut nodes = Vec::new();

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

impl ModuleInitDefinition {
    /// Evaluate a call to the module init definition
    pub fn eval_to_node(
        &self,
        args: &ArgumentMap,
        object_builder: &mut ObjectBuilder,
        context: &mut Context,
    ) -> EvalResult<()> {
        assert!(matches!(context.top(), StackFrame::Module(_, _)));

        context.open_module_init(args.into());

        let mut nodes = Vec::new();

        for statement in &self.body.statements {
            match statement {
                Statement::Assignment(assignment) => {
                    let id = &assignment.id;
                    let value = assignment.expression.eval(context)?;

                    todo!()
                }
                Statement::Expression(expression) => {
                    object_builder.append_children(&mut expression.eval(context)?.fetch_nodes());
                }
                _ => {
                    context.error(self, EvalError::StatementNotSupported(statement.clone()))?;
                }
            }
        }
        context.close();
    }
}
