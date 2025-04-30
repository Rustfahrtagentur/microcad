// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module definition syntax element

use crate::{diag::*, eval::*, objects::*, rc::*, resolve::*, src_ref::*, syntax::*, value::*};

/// Module definition
#[derive(Clone, Debug)]
pub struct ModuleDefinition {
    /// Module name
    pub id: Identifier,
    /// Module parameters (implicit initialization)
    pub parameters: ParameterList,
    /// Module body
    pub body: Body,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl ModuleDefinition {
    /// Return iterator over all initializers
    pub fn inits(&self) -> Inits {
        Inits::new(self)
    }
    /// Find a matching initializer for call argument list
    fn find_matching_initializer(
        &self,
        args: &CallArgumentList,
        context: &mut EvalContext,
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
    pub fn resolve(self: &Rc<Self>, parent: Option<SymbolNodeRcMut>) -> RcMut<SymbolNode> {
        let node = SymbolNode::new(SymbolDefinition::Module(self.clone()), parent);
        node.borrow_mut().children = self.body.resolve(Some(node.clone()));
        node
    }

    /// Try to evaluate a single call to an object
    fn eval_to_node<'a>(
        &'a self,
        args: &ArgumentMap,
        init: Option<&'a ModuleInitDefinition>,
        context: &mut EvalContext,
    ) -> EvalResult<ObjectNode> {
        let mut props = ObjectProperties::from_parameter_list(&self.parameters, context)?;
        context.open_scope();

        // Create the object node from initializer if present
        let object = match init {
            Some(init) => init.eval_to_node(args, props, context)?,
            None => {
                // Add values from argument map as local values
                for (id, value) in args.iter() {
                    props.assign_and_add_local_value(id, value.clone(), context);
                }
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
        let mut nodes = Vec::new();

        for statement in &self.body.statements {
            match statement {
                Statement::Assignment(assignment) => {
                    let id = assignment.name.id();
                    let value = assignment.value.eval(context)?;
                    context.add_local_value(id.clone(), value)?;
                }
                Statement::Expression(expression) => {
                    let value = expression.eval(context)?;
                    nodes.append(&mut value.fetch_nodes());
                }
                _ => {}
            }
        }

        context.close();

        for node in nodes {
            object.append(node);
        }

        Ok(object)
    }

    /// Evaluate the call of a module
    ///
    /// The evaluation considers multiplicity, which means that multiple nodes maybe created.
    ///
    /// Example:
    /// Consider the `module a(b: Scalar) { }`.
    /// Calling the module `a([1.0, 2.0])` results in two nodes with `b = 1.0` and `b = 2.0`, respectively.
    pub fn eval_call(
        &self,
        args: &CallArgumentList,
        context: &mut EvalContext,
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

        Ok(Value::NodeMultiplicity(nodes))
    }
}

/// Iterator over modules init statements
pub struct Inits<'a>(std::slice::Iter<'a, Statement>);

impl<'a> Inits<'a> {
    fn new(def: &'a ModuleDefinition) -> Self {
        Self(def.body.statements.iter())
    }
}

impl<'a> Iterator for Inits<'a> {
    type Item = &'a ModuleInitDefinition;

    fn next(&mut self) -> Option<Self::Item> {
        for statement in self.0.by_ref() {
            match statement {
                Statement::ModuleInit(module_init_definition) => {
                    return Some(module_init_definition);
                }
                _ => continue,
            }
        }

        None
    }
}

impl SrcReferrer for ModuleDefinition {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for ModuleDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "module {name}({parameters}) {body}",
            name = self.id,
            parameters = self.parameters,
            body = self.body
        )
    }
}

impl PrintSyntax for ModuleDefinition {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}ModuleDefinition '{}':", "", self.id)?;
        self.parameters.print_syntax(f, depth + 1)?;
        self.body.print_syntax(f, depth + 1)
    }
}
