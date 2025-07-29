// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::*};

impl Parse for Body {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::body);
        Ok(Body {
            statements: crate::find_rule!(pair, statement_list)?,
            src_ref: pair.into(),
        })
    }
}
