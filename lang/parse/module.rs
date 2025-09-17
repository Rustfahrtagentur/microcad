// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, rc::*, syntax::*};

impl Parse for Rc<ModuleDefinition> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let body = if let Ok(body) = crate::find_rule!(pair, body) {
            Some(body)
        } else {
            None
        };

        Ok(Rc::new(ModuleDefinition {
            visibility: crate::find_rule!(pair, visibility)?,
            id: crate::find_rule!(pair, identifier)?,
            body,
            src_ref: pair.clone().into(),
        }))
    }
}
