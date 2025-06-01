// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::*};

impl Parse for Body {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rules(&pair, &[Rule::body, Rule::body_else]);
        Ok(Body {
            statements: pair.find(Rule::statement_list).unwrap_or_default(),
            src_ref: pair.into(),
        })
    }
}
