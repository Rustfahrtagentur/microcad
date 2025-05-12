// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, rc::*, syntax::*};

impl Parse for Rc<ModuleDefinition> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut name = Identifier::default();
        let mut parameters = ParameterList::default();
        let mut parameters_src_ref = SrcRef(None);
        let mut body = Body::default();

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::identifier => {
                    name = Identifier::parse(pair)?;
                }
                Rule::parameter_list => {
                    parameters = ParameterList::parse(pair.clone())?;
                    parameters_src_ref = pair.into();
                }
                Rule::body => {
                    body = Body::parse(pair.clone())?;
                }
                rule => unreachable!("Unexpected rule for module definition, got {:?}", rule),
            }
        }

        Ok(Rc::new(ModuleDefinition {
            id: name,
            explicit: Rc::new(ModuleInitDefinition {
                parameters,
                body: Body::default(),
                src_ref: parameters_src_ref,
            }),
            body,
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
