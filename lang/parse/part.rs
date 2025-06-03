// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, rc::*, syntax::*};

impl Parse for Rc<PartDefinition> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(Rc::new(PartDefinition {
            attribute_list: pair.find(Rule::attribute_list).unwrap_or_default(),
            id: pair.find(Rule::identifier).expect("Part id"),
            parameters: pair.find(Rule::parameter_list).expect("Parameters"),
            body: pair.find(Rule::body).expect("Part body"),
            src_ref: pair.into(),
        }))
    }
}

impl Parse for InitDefinition {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::init_definition);

        Ok(InitDefinition {
            parameters: pair.find(Rule::parameter_list).unwrap_or_default(),
            body: pair.find(Rule::body).unwrap_or_default(),
            src_ref: pair.into(),
        })
    }
}
