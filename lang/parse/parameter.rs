// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*};

/// Short cut to create a `ParameterList` instance
impl Parse for Parameter {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::parameter);
        Ok(Self {
            id: pair.find(Rule::identifier).expect("Identifier"),
            specified_type: pair
                .find(Rule::r#type)
                .map(|ty| TypeAnnotation(Refer::new(ty, pair.src_ref()))),
            default_value: pair.find(Rule::expression),
            src_ref: pair.into(),
        })
    }
}

impl Parse for ParameterList {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::parameter_list);
        let mut parameters = ParameterList::default();

        for pair in pair.inner() {
            if pair.as_rule() == Rule::parameter {
                parameters
                    .try_push(Parameter::parse(pair)?)
                    .map_err(ParseError::DuplicateIdentifier)?;
            }
        }

        Ok(parameters)
    }
}
