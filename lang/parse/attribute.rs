// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*};

impl Parse for AttributeCommand {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::attribute_command);

        for inner in pair.inner() {
            match inner.as_rule() {
                Rule::expression => return Ok(Self::Expression(Expression::parse(inner)?)),
                Rule::identifier | Rule::argument_list => {
                    return Ok(Self::Call(
                        pair.find(Rule::identifier),
                        pair.find(Rule::argument_list),
                    ));
                }
                _ => {}
            }
        }

        unreachable!()
    }
}

impl Parse for Attribute {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rules(&pair, &[Rule::attribute, Rule::inner_attribute]);

        let mut commands = Vec::new();
        for pair in pair.inner() {
            if pair.as_rule() == Rule::attribute_command {
                commands.push(AttributeCommand::parse(pair)?);
            }
        }

        Ok(Self {
            id: pair.find(Rule::identifier).expect("Id"),
            commands,
            is_inner: pair.as_rule() == Rule::inner_attribute,
            src_ref: pair.src_ref(),
        })
    }
}

impl Parse for AttributeList {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::attribute_list);
        let mut attribute_list = AttributeList::default();

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::attribute => {
                    attribute_list.push(Attribute::parse(pair)?);
                }
                Rule::COMMENT | Rule::doc_comment => {}
                rule => unreachable!("Unexpected element {rule:?}"),
            }
        }

        Ok(attribute_list)
    }
}
