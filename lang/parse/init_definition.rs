// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::*};

impl Parse for InitDefinition {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::init_definition);

        Ok(InitDefinition {
            parameters: crate::find_rule!(pair, parameter_list)?,
            body: crate::find_rule!(pair, body)?,
            src_ref: pair.into(),
        })
    }
}
