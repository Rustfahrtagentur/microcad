// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module initialization definition parser entity

use crate::{eval::*, objects::*, parse::*, parser::*, src_ref::*, sym::*};

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
        context: &mut EvalContext,
        node: &mut ObjectNode,
    ) -> EvalResult<()> {
        // Copy the arguments to the symbol table of the node
        for (name, value) in arg_map.iter() {
            node.add(Symbol::Value(name.clone(), value.clone()));
        }

        for (name, value) in arg_map.iter() {
            context.add(Symbol::Value(name.clone(), value.clone()));
        }

        let node_body = self.body.eval(context)?;

        // Add the init object's children to the node
        for child in node_body.children() {
            child.detach();
            node.append(child.clone());
        }
        node_body.copy(node)?;

        // Now, copy the symbols of the node into the context
        node.copy(context)?;

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
