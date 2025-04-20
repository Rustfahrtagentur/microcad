// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module definition syntax element

use crate::{diag::PushDiag, eval::*, objects::ObjectNode, src_ref::*, syntax::*, value::Value};

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
    fn find_matching_initializer(&self, args: &CallArgumentList, context: &mut EvalContext) -> Option<(&ModuleInitDefinition, MultiArgumentMap)> {
        self.inits().find_map(|init| {
            if let Ok(arg_map) = args.get_multi_matching_arguments(context, &init.parameters) {
                Some((init, arg_map))
            } else {
                None
            }
        })
    }

    /// Try to evaluate a single call to an object
    fn eval_to_node<'a>(&'a self, args: &ArgumentMap, init: Option<&'a ModuleInitDefinition>, context: &mut EvalContext) -> EvalResult<ObjectNode> {
        let mut props = SortedValueList::from_parameter_list(&self.parameters, context)?;

        use crate::objects::*;

        context.open_scope();

        // Create the object node from initializer if present
        let object = match init {
            Some(init) => init.eval_to_node(args, props, context)?,
            None => {
                // Add values from argument map as local values
                for (id, value) in args.iter() {
                    props.assign_and_add_local_value(id, value.clone(), context);
                }
                if !props.is_complete() {
                    use crate::diag::PushDiag;
                    context.error(self, EvalError::UninitializedProperties(props.get_incomplete_ids()))?;
                    return Ok(crate::objects::empty_object());
                } 
        
                object(Object { props })
            }
        };

        // At this point, all properties must have a value 
        let mut nodes = Vec::new();

        for statement in &self.body.statements {
            match statement {
                Statement::Assignment(assignment) => {
                    let id = assignment.name.id();
                    let value = assignment.value.eval(context)?;
                    context.add_local_value(id.clone(), value);
                }
                Statement::Expression(expression) => {
                    if let Value::Node(node) = expression.eval(context)? { nodes.push(node) }
                }
                _ => {}
            }
        }

        context.close_scope();

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
    pub fn eval_call(&self, args: &CallArgumentList, context: &mut EvalContext) -> EvalResult<Vec<ObjectNode>> {
        let mut nodes = Vec::new();

        match self.find_matching_initializer(args, context) {
            Some((init, multi_args)) => {
            // We have a found a matching initializer. Evaluate each argument combination into a node 
            for args in multi_args.combinations() {
                    nodes.push(self.eval_to_node(&args, Some(init), context)?);
                }
            }
            None => {
                match args.get_multi_matching_arguments(context, &self.parameters) {
                    Ok(multi_args) => {
                        for args in multi_args.combinations() {
                            nodes.push(self.eval_to_node(&args, None, context)?);
                        }        
                    }
                    Err(err) => {
                        context.error(self, err)?;
                    }   
                }
            }
        }


        Ok(nodes)
    }
}



/// A list of module property values
/// 
/// It is required that properties are always sorted by their id. 
#[derive(Clone, Default, Debug)]
pub struct SortedValueList(Vec<(crate::Id, Value)>);

impl SortedValueList {
    /// Create initial property list by evaluating parameter list
    pub fn from_parameter_list(parameter_list: &ParameterList, context: &mut EvalContext) -> EvalResult<Self> {
        let mut props = Vec::new();
        for parameter in parameter_list.iter() {
            props.push((parameter.name.id().clone(), parameter.eval_default_value(context)?));
        }

        props.sort_by(|(lhs, _), (rhs, _)| lhs.cmp(rhs));

        Ok(Self(props))
    }

    /// Test if each property has a value
    pub fn is_complete(&self) -> bool {
        self.0.iter().all(|(_, value)| !value.is_invalid())
    }

    /// Get index of item
    pub fn get_index(&self, id: &crate::Id) -> Result<usize, usize> {
        self.0.binary_search_by(|(prop_id, _)| prop_id.cmp(id))
    }

    /// Get value at id
    pub fn get_value(&self, id: &crate::Id) -> Option<&Value> {
        match self.get_index(id) {
            Ok(index) => self.0.get(index).map(|p| &p.1),
            Err(_) => None,
        }
    }

    /// Get mutable value at id
    pub fn get_value_mut(&mut self, id: &crate::Id) -> Option<&mut Value> {
        match self.get_index(id) {
            Ok(index) => self.0.get_mut(index).map(|p| &mut p.1),
            Err(_) => None,
        }
    }

    /// Get all ids where `value == Value::None`
    pub fn get_incomplete_ids(&self) -> Vec<crate::Id> {
        self.0.iter().filter_map(|(id, value)| if value.is_invalid() { Some(id.clone()) } else {None}).collect()
    }

    /// Assign the new value to the respective id if present and add as local value to the context 
    pub fn assign_and_add_local_value(&mut self, id: &crate::Id, value: Value, context: &mut EvalContext) {
        if let Some(prop_value) = self.get_value_mut(id) {
            *prop_value = value.clone();
        }
        
        // The result of the assignment becomes a local value, too
        context.add_local_value(id.clone(), value);
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
