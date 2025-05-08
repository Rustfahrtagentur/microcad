// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, rc::*, syntax::*};

impl Parse for Rc<NamespaceDefinition> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut pairs = pair.inner();
        Ok(Rc::new(NamespaceDefinition {
            id: Identifier::parse(pairs.next().expect("Identifier expected"))?,
            body: Body::parse(pairs.next().expect("NamespaceBody expected"))?,
            src_ref: pair.clone().into(),
        }))
    }
}
