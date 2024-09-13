// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Use alias parser entity

use crate::{errors::*, parse::*, parser::*, src_ref::*};

/// Use alias
#[derive(Clone, Debug)]
pub struct UseAlias(pub QualifiedName, pub Identifier, SrcRef);

impl SrcReferrer for UseAlias {
    fn src_ref(&self) -> SrcRef {
        self.2.clone()
    }
}

impl std::fmt::Display for UseAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "use {:?} as {:?}", self.0, self.1)
    }
}

impl Parse for UseAlias {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let mut inner = pair.inner();
        Ok(UseAlias(
            QualifiedName::parse(inner.next().unwrap())?,
            Identifier::parse(inner.next().unwrap())?,
            pair.src_ref(),
        ))
    }
}
