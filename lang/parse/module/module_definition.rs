// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module definition parser entity

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};

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
        ModuleDefinition {
            name,
            body: ModuleDefinitionBody::default(),
            src_ref: SrcRef(None),
        }
    }
}

impl CallTrait for ModuleDefinition {
    fn call(&self, args: &CallArgumentList, context: &mut Context) -> Result<Option<Value>> {
        let stack_frame = StackFrame::ModuleCall(context.top().symbol_table().clone(), None);

        let mut node = crate::objecttree::group();

        context.scope(stack_frame, |context| {
            // Let's evaluate the pre-init statements first
            for statement in &self.body.pre_init_statements {
                statement.eval(context)?;
            }

            let mut matching_init = Vec::new();
            let arg_values = args.eval(context)?;

            // Find all initializers that match the arguments and add it to the matching_init list
            for init in &self.body.inits {
                let param_values = init.parameters.eval(context)?;
                if let Ok(arg_map) = arg_values.get_matching_arguments(&param_values) {
                    matching_init.push((init, arg_map));
                }
            }

            use crate::diag::PushDiag;
            use anyhow::anyhow;

            // There should be only one matching initializer
            match matching_init.len() {
                0 => {
                    context.error(self, anyhow!("No matching initializer found"))?;
                }
                1 => {
                    let (init, arg_map) = matching_init.first().unwrap();
                    // Copy the arguments to the symbol table of the node
                    for (name, value) in arg_map.iter() {
                        node.add(Symbol::Value(name.clone(), value.clone()));
                    }
                    let init_object = init.call(arg_map, context)?;

                    // Add the init object's children to the node
                    for child in init_object.children() {
                        child.detach();
                        node.append(child.clone());
                    }

                    init_object.copy(&mut node);
                }
                _ => {
                    context.error(self, anyhow!("Multiple matching initializers found"))?;
                    // TODO Add diagnostics for multiple matching initializers
                    return Ok(());
                }
            }

            // Now, copy the symbols of the node into the context
            node.copy(context);

            // Evaluate the post-init statements
            for statement in &self.body.post_init_statements {
                if let Some(Value::Node(new_child)) = statement.eval(context)? {
                    node.append(new_child);
                }
            }

            Ok(())
        })?;

        Ok(Some(Value::Node(node)))
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
