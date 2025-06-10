// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::*};

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
