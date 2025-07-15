// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*};

impl Parse for Attribute {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rules(
            &pair,
            &[Rule::attribute_name_value, Rule::attribute_named_tuple],
        );

        match pair.as_rule() {
            Rule::attribute_name_value => Ok(Self::NameValue(
                crate::find_rule!(pair, identifier)?,
                crate::find_rule!(pair, expression)?,
            )),
            Rule::attribute_named_tuple => Ok(Self::Tuple(
                crate::find_rule!(pair, identifier)?,
                crate::find_rule!(pair, argument_list)?,
            )),
            _ => unreachable!(),
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
                Rule::COMMENT | Rule::doc_comment => {}
                rule => unreachable!("Unexpected element {rule:?}"),
            }
        }

        Ok(attribute_list)
    }
}
