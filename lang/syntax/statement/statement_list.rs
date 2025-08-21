// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Statement list syntax element.

use crate::syntax::*;
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
