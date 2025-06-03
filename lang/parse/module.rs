// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, rc::*, syntax::*};

impl Parse for Rc<ModuleDefinition> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut pairs = pair.inner();
        Ok(Rc::new(ModuleDefinition {
            id: Identifier::parse(pairs.next().expect("identifier"))?,
            body: Body::parse(pairs.next().expect("module body"))?,
            src_ref: pair.clone().into(),
        }))
    }
}
