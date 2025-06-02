// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, rc::*, syntax::*};

impl Parse for Rc<ModuleDefinition> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(Rc::new(ModuleDefinition {
            attribute_list: pair.find(Rule::attribute_list).unwrap_or_default(),
            id: pair.find(Rule::identifier).expect("Module id"),
            parameters: pair.find(Rule::parameter_list).expect("Parameters"),
            body: pair.find(Rule::body).expect("Module body"),
            src_ref: pair.into(),
        }))
    }
}

impl Parse for ModuleInitDefinition {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::module_init_definition);

        Ok(ModuleInitDefinition {
            parameters: pair.find(Rule::parameter_list).unwrap_or_default(),
            body: pair.find(Rule::body).unwrap_or_default(),
            src_ref: pair.into(),
        })
    }
}
