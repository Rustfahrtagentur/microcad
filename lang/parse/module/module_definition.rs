// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module definition parser entity

use crate::{eval::*, objecttree, parse::*, parser::*, src_ref::*, ObjectNode};

/// Module definition
#[derive(Clone, Debug)]
pub struct ModuleDefinition {
    /// Module name
    pub name: Identifier,
    /// Module body
    pub body: ModuleDefinitionBody,
    /// Source code reference
    src_ref: SrcRef,
}

impl ModuleDefinition {
    /// Create new module definition
    pub fn new(name: Identifier) -> Self {
        Self {
            name,
            body: ModuleDefinitionBody::default(),
            src_ref: SrcRef(None),
        }
    }

    /// Call the initializer with the given argument map
    pub fn call_init(
        &self,
        init: std::rc::Rc<ModuleInitDefinition>,
        multi_argument_map: &MultiArgumentMap,
        context: &mut Context,
    ) -> EvalResult<ObjectNode> {
        let mut group = objecttree::group();

        for arg_map in multi_argument_map.combinations() {
            // Copy the arguments to the symbol table of the node
            for (name, value) in arg_map.iter() {
                group.add(Symbol::Value(name.clone(), value.clone()));
            }
            let init_object = init.call(&arg_map, context)?;

            // Add the init object's children to the node
            for child in init_object.children() {
                child.detach();
                group.append(child.clone());
            }
            init_object.copy(&mut group);

            // Now, copy the symbols of the node into the context
            group.copy(context);

            // Evaluate the post-init statements
            for statement in &self.body.post_init_statements {
                match statement {
                    ModuleDefinitionStatement::Assignment(assignment) => {
                        // Evaluate the assignment and add the symbol to the node
                        // E.g. `a = 1` will add the symbol `a` to the node
                        let symbol = assignment.eval(context)?;
                        group.add(symbol);
                    }
                    statement => {
                        if let Some(Value::Node(new_child)) = statement.eval(context)? {
                            group.append(new_child);
                        }
                    }
                }
            }
        }

        Ok(group)
    }

    /// Find the matching initializer for the given call argument value list
    pub fn find_matching_initializer(
        &self,
        call_argument_list: &CallArgumentList,
        context: &mut Context,
    ) -> EvalResult<(std::rc::Rc<ModuleInitDefinition>, MultiArgumentMap)> {
        let call_argument_value_list = call_argument_list.eval(context)?;

        for init in &self.body.inits {
            match call_argument_value_list
                .get_multi_matching_arguments(&init.parameters.eval(context)?)
            {
                Ok(multi_argument_map) => return Ok((init.clone(), multi_argument_map)),
                Err(_) => continue,
            }
        }

        Err(EvalError::NoMatchingInitializer(self.name.clone()))
    }
}

impl CallTrait for ModuleDefinition {
    type Output = Vec<ObjectNode>;

    fn call(
        &self,
        call_argument_list: &CallArgumentList,
        context: &mut Context,
    ) -> EvalResult<Self::Output> {
        use crate::diag::PushDiag;

        let stack_frame = StackFrame::ModuleCall(context.top().symbol_table().clone(), None);
        let mut nodes = Vec::new();

        context.scope(stack_frame, |context| {
            match self.find_matching_initializer(call_argument_list, context) {
                Ok((init, multi_argument_map)) => {
                    // Let's evaluate the pre-init statements first (they are evaluated before the initializer)
                    for statement in &self.body.pre_init_statements {
                        statement.eval(context)?;
                    }

                    // Call the initializer
                    let node = self.call_init(init, &multi_argument_map, context)?;
                    nodes.push(node);
                }
                Err(err) => {
                    context.error(self, Box::new(err))?;
                }
            }
            Ok(())
        })?;

        Ok(nodes)
    }
}

impl SrcReferrer for ModuleDefinition {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Symbols for ModuleDefinition {
    fn fetch(&self, id: &Id) -> Option<std::rc::Rc<Symbol>> {
        self.body.fetch(id)
    }

    fn add(&mut self, symbol: Symbol) -> &mut Self {
        self.body.add(symbol);
        self
    }

    fn add_alias(&mut self, symbol: Symbol, alias: Id) -> &mut Self {
        self.body.add_alias(symbol, alias);
        self
    }

    fn copy<T: Symbols>(&self, into: &mut T) {
        self.body.symbols.iter().for_each(|(_, symbol)| {
            into.add(symbol.as_ref().clone());
        });
    }
}

impl Parse for std::rc::Rc<ModuleDefinition> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut name = Identifier::default();
        let mut parameters = None;
        let mut body = ModuleDefinitionBody::default();

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::identifier => {
                    name = Identifier::parse(pair)?;
                }
                Rule::parameter_list => {
                    parameters = Some(ParameterList::parse(pair)?);
                }
                Rule::module_definition_body => {
                    body = ModuleDefinitionBody::parse(pair.clone())?;
                }
                rule => unreachable!("Unexpected rule for module definition, got {:?}", rule),
            }
        }

        if let Some(parameters) = parameters {
            body.add_initializer_from_parameter_list(parameters)?;
        }

        Ok(std::rc::Rc::new(ModuleDefinition {
            name,
            body,
            src_ref: pair.into(),
        }))
    }
}
