// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Statement list syntax element.

use crate::{src_ref::*, syntax::*};
use derive_more::Deref;

/// A list of statements.
#[derive(Clone, Default, Debug, Deref)]
pub struct StatementList(pub Vec<Statement>);

impl std::fmt::Display for StatementList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in self.iter() {
            writeln!(f, "{statement}")?;
        }
        Ok(())
    }
}

impl SrcReferrer for StatementList {
    fn src_ref(&self) -> SrcRef {
        if let (Some(first), Some(last)) = (self.first(), self.last()) {
            SrcRef::merge(first, last)
        } else {
            SrcRef(None)
        }
    }
}
