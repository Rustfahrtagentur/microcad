// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::DocBlock};

impl Parse for DocBlock {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::doc_block);
        let mut lines = Vec::new();
        for pair in pair.inner() {
            if pair.as_rule() == Rule::doc_comment {
                lines.push(String::from(
                    pair.as_str()
                        .trim()
                        .strip_prefix("/// ")
                        .unwrap_or_default(),
                ));
            }
        }

        Ok(Self {
            lines,
            src_ref: pair.src_ref(),
        })
    }
}
