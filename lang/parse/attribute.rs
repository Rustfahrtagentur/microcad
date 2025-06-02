// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*};

impl Parse for Attribute {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::attribute_item);

        if let Some(call) = pair.find(Rule::call) {
            return Ok(Self::Call(call));
        }

        if let Some(qualified_name) = pair.find(Rule::qualified_name) {
            if let Some(expression) = pair.find(Rule::expression) {
                Ok(Self::NameValue(qualified_name, expression))
            } else {
                Ok(Self::Tag(qualified_name))
            }
        } else {
            unreachable!("Invalid attribute")
        }
    }
}

impl Parse for AttributeList {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::attribute_list);

        let mut attribute_list = AttributeList::default();

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::attribute => {
                    for pair in pair.inner() {
                        attribute_list.push(Attribute::parse(pair)?);
                    }
                }
                rule => unreachable!("Unexpected element {rule:?}"),
            }
        }

        Ok(attribute_list)
    }
}
