// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module initialization definition parser entity

use crate::{eval::*, parse::*, parse::*, parser::*, src_ref::*};

/// Module initialization definition
///
/// Example:
///
/// ```uCAD
/// module a {
///     init(b: length) {} // The init definition
/// }
/// ```
#[derive(Clone, Debug)]
pub struct ModuleInitDefinition {
    /// Parameter list for this init definition
    pub parameters: ParameterList,
    /// Body if the init definition
    pub body: NodeBody,
    /// Source reference
    pub src_ref: SrcRef,
}

impl ModuleInitDefinition {
    /// Call the initializer
    pub fn call(
        &self,
        arg_map: &ArgumentMap,
        context: &mut Context,
    ) -> Result<crate::objecttree::ObjectNode> {
        for (name, value) in arg_map.iter() {
            context.add(Symbol::Value(name.clone(), value.clone()));
        }

        self.body.eval(context)
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

        Ok(ModuleInitDefinition {
            parameters: pair.find(Rule::parameter_list).unwrap_or_default(),
            body: pair.find(Rule::node_body).unwrap_or_default(),
            src_ref: pair.into(),
        })
    }
}

impl std::fmt::Display for ModuleInitDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "init({parameters}) ", parameters = self.parameters)?;
        write!(f, "{body}", body = self.body)
    }
}
