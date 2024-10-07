// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module definition parser entity

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};

/// Module definition
#[derive(Clone, Debug)]
pub struct ModuleDefinition {
    /// Module attributes
    pub attributes: Vec<Attribute>,
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
            attributes: Vec::new(),
            name,
            body: ModuleDefinitionBody::default(),
            src_ref: SrcRef(None),
        }
    }
}

impl CallTrait for ModuleDefinition {
    fn call(&self, args: &CallArgumentList, context: &mut Context) -> Result<Option<Value>> {
        context.push();

        let node = microcad_render::tree::group();
        context.set_current_node(node.clone());

        // Lets evaluate the pre-init statements
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
                init.call(arg_map, context)?;
            }
            _ => {
                context.error(self, anyhow!("Multiple matching initializers found"))?;
                // TODO Add diagnostics for multiple matching initializers
            }
        }

        for statement in &self.body.post_init_statements {
            statement.eval(context)?;
        }

        context.pop();

        Ok(Some(Value::Node(node)))
    }
}

impl SrcReferrer for ModuleDefinition {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Symbols for ModuleDefinition {
    fn fetch(&self, id: &Id) -> Vec<&Symbol> {
        self.body.fetch(id)
    }

    fn add(&mut self, symbol: Symbol) -> &mut Self {
        self.body.add(symbol);
        self
    }
    fn copy<T: Symbols>(&self, into: &mut T) {
        self.body.symbols.iter().for_each(|symbol| {
            into.add(symbol.clone());
        });
    }
}

impl Parse for std::rc::Rc<ModuleDefinition> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut attributes = Vec::new();
        let mut name = Identifier::default();
        let mut parameters = None;
        let mut body = ModuleDefinitionBody::default();

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::attribute_list => {
                    attributes.push(Attribute::parse(pair)?);
                }
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
            attributes,
            name,
            body,
            src_ref: pair.into(),
        }))
    }
}
