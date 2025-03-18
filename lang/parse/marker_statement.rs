// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Node marker parser entity

use crate::{parse::*, parser::*, src_ref::*};

/// Node marker, e.g. `@children`
#[derive(Clone, Debug)]
pub struct Marker {
    /// Marker name, e.g. `children`
    pub name: Identifier,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl Marker {
    /// Returns true if the marker is a children marker
    pub fn is_children_marker(&self) -> bool {
        &self.name == "children"
    }
}

impl SrcReferrer for Marker {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.src_ref.clone()
    }
}

impl Parse for Marker {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::marker_statement);
        Ok(Self {
            name: Identifier::parse(pair.inner().next().expect(INTERNAL_PARSE_ERROR))?,
            src_ref: pair.src_ref(),
        })
    }
}

impl std::fmt::Display for Marker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.name)
    }
}
