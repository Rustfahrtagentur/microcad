// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module initialization definition parser entity

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};

/// Module initialization definition
#[derive(Clone, Debug)]
pub struct ModuleInitDefinition {
    pub parameters: ParameterList,
    pub body: Vec<ModuleInitStatement>,
    pub src_ref: SrcRef,
}

impl ModuleInitDefinition {
    pub fn new(parameters: ParameterList, body: Vec<ModuleInitStatement>) -> Self {
        Self {
            parameters,
            body,
            src_ref: SrcRef(None),
        }
    }

    pub fn call(&self, arg_map: &ArgumentMap, context: &mut Context) -> Result<()> {
        for (name, value) in arg_map.iter() {
            context.add(Symbol::Value(name.clone(), value.clone()));
        }

        for statement in &self.body {
            statement.eval(context)?;
        }
        Ok(())
    }
}

impl SrcReferrer for ModuleInitDefinition {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Parse for ModuleInitDefinition {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::module_init_definition);
        let mut parameters = ParameterList::default();
        let mut body = Vec::new();

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::parameter_list => {
                    parameters = ParameterList::parse(pair)?;
                }
                Rule::module_init_statement => {
                    body.push(ModuleInitStatement::parse(pair)?);
                }
                Rule::COMMENT => {}
                rule => unreachable!(
                    "expected parameter_list or module_init_statement. Instead found {rule:?}"
                ),
            }
        }

        Ok(ModuleInitDefinition {
            parameters,
            body,
            src_ref: pair.into(),
        })
    }
}
