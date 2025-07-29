// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, rc::*, syntax::*};

impl Parse for Rc<FunctionDefinition> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::function_definition);

        Ok(Rc::new(FunctionDefinition {
            visibility: crate::find_rule!(pair, visibility)?,
            id: crate::find_rule!(pair, identifier)?,
            signature: pair
                .find(Rule::function_signature)
                .expect("Function signature"),
            body: crate::find_rule!(pair, body)?,
            src_ref: pair.clone().into(),
        }))
    }
}

impl Parse for FunctionSignature {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut parameters = ParameterList::default();
        let mut return_type = None;

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::parameter_list => {
                    parameters = ParameterList::parse(pair)?;
                }
                Rule::r#type => return_type = Some(TypeAnnotation::parse(pair)?),
                rule => unreachable!("Unexpected token in function signature: {:?}", rule),
            }
        }

        Ok(Self {
            parameters,
            return_type,
            src_ref: pair.into(),
        })
    }
}
