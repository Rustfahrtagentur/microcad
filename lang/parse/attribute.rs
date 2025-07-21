// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*};

impl Parse for AttributeSubcommand {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::attribute_subcommand);

        for inner in pair.inner() {
            match inner.as_rule() {
                Rule::format_string => return Ok(Self::FormatString(FormatString::parse(inner)?)),
                Rule::identifier => {
                    return Ok(Self::Call(
                        pair.find(Rule::identifier).expect("Identifier"),
                        pair.find(Rule::argument_list),
                    ));
                }
                _ => {}
            }
        }

        unreachable!()
    }
}

impl Parse for AttributeCommand {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::attribute_command);

        // TODO: Use try_collect() once this iterator is a stable feature.
        let mut subcommands = Vec::new();
        for pair in pair.inner() {
            if pair.as_rule() == Rule::attribute_subcommand {
                subcommands.push(AttributeSubcommand::parse(pair)?);
            }
        }

        Ok(Self {
            id: pair.find(Rule::identifier).expect("Identifier"),
            subcommands,
            src_ref: pair.src_ref(),
        })
    }
}

impl Parse for Attribute {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::attribute);

        let inner = pair.inner().next().expect("pair");

        Ok(match inner.as_rule() {
            Rule::attribute_name_value => Self::NameValue(
                inner.find(Rule::identifier).expect("Identifier"),
                inner.find(Rule::expression).expect("Expression"),
            ),
            Rule::attribute_exporter => Self::Exporter(
                inner.find(Rule::identifier).expect("Identifier"),
                inner.find(Rule::argument_list).unwrap_or_default(),
            ),
            Rule::attribute_command => Self::Command(
                pair.find(Rule::attribute_command)
                    .expect("Attribute command"),
            ),
            _ => unreachable!(),
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
