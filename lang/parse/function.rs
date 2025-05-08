// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, rc::*, syntax::*};

impl Parse for Rc<FunctionDefinition> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::function_definition);
        let mut inner = pair.inner();
        let name = Identifier::parse(inner.next().expect(INTERNAL_PARSE_ERROR))?;
        let signature = FunctionSignature::parse(inner.next().expect(INTERNAL_PARSE_ERROR))?;
        let body = Body::parse(inner.next().expect(INTERNAL_PARSE_ERROR))?;

        Ok(Rc::new(FunctionDefinition {
            id: name,
            signature,
            body,
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
