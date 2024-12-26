// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module definition parser entity

use crate::{eval::*, objects::*, parse::*, parser::*, src_ref::*, sym::*};

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

    /// Find the matching initializer for the given call argument value list
    fn find_matching_initializer(
        &self,
        call_argument_list: &CallArgumentList,
        context: &mut Context,
    ) -> EvalResult<InitializerMatch> {
        let call_argument_value_list = call_argument_list.eval(context)?;

        // check for any implicit initializer
        if let Some(init) = &self.body.implicit_init {
            if let Ok(multi_argument_map) = call_argument_value_list
                .get_multi_matching_arguments(&init.parameters.eval(context)?)
            {
                return Ok(InitializerMatch::Implicit(init.clone(), multi_argument_map));
            }
        }

        // find match in the explicit Initializers
        for init in &self.body.explicit_inits {
            match call_argument_value_list
                .get_multi_matching_arguments(&init.parameters.eval(context)?)
            {
                Ok(multi_argument_map) => {
                    return Ok(InitializerMatch::Explicit(init.clone(), multi_argument_map))
                }
                Err(_) => continue,
            }
        }

        Err(EvalError::NoMatchingInitializer(self.name.clone()))
    }
}

/// Match of an initializer
///
/// This enum represents a match of an initializer containing the initializer itself and the argument map
enum InitializerMatch {
    /// Match of an implicit initializer
    Implicit(std::rc::Rc<ModuleInitDefinition>, MultiArgumentMap),

    /// Match of an explicit initializer
    Explicit(std::rc::Rc<ModuleInitDefinition>, MultiArgumentMap),
}

impl InitializerMatch {
    /// Call the initializer and the pre-init and post-init statements
    fn call(
        &self,
        context: &mut Context,
        body: &ModuleDefinitionBody,
    ) -> EvalResult<Vec<ObjectNode>> {
        let mut nodes = Vec::new();

        match self {
            InitializerMatch::Implicit(init, multi_argument_map) => {
                for arg_map in multi_argument_map.combinations() {
                    let mut group: rctree::Node<ObjectNodeInner> = group();

                    init.call(&arg_map, context, &mut group)?;
                    body.eval_pre_init_statements(context, &mut group)?;
                    body.eval_post_init_statements(context, &mut group)?;

                    nodes.push(group);
                }
            }
            InitializerMatch::Explicit(init, multi_argument_map) => {
                for arg_map in multi_argument_map.combinations() {
                    let mut group: rctree::Node<ObjectNodeInner> = group();

                    body.eval_pre_init_statements(context, &mut group)?;
                    init.call(&arg_map, context, &mut group)?;
                    body.eval_post_init_statements(context, &mut group)?;

                    nodes.push(group);
                }
            }
        }

        Ok(nodes)
    }
}

impl CallTrait for std::rc::Rc<ModuleDefinition> {
    type Output = Vec<ObjectNode>;

    fn call(
        &self,
        call_argument_list: &CallArgumentList,
        context: &mut Context,
    ) -> EvalResult<Self::Output> {
        let stack_frame = StackFrame::module(context, self.clone())?;

        context.scope(stack_frame, |context| {
            match self.find_matching_initializer(call_argument_list, context) {
                Ok(matching_initializer) => Ok(matching_initializer.call(context, &self.body)?),
                Err(err) => {
                    use crate::diag::PushDiag;
                    context.error(self.as_ref(), Box::new(err), Some(context.stack_trace()))?;
                    Ok(Vec::new())
                }
            }
        })
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

    fn copy<T: Symbols>(&self, into: &mut T) -> SymResult<()> {
        self.body.symbols.iter().for_each(|(_, symbol)| {
            into.add(symbol.as_ref().clone());
        });
        Ok(())
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
