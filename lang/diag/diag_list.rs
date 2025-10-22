// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{diag::*, resolve::*};
use derive_more::Deref;

/// Source file diagnostics.
#[derive(Debug, Default, Deref)]
pub struct DiagList(Vec<Diagnostic>);

impl DiagList {
    /// Clear diagnostics.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Pretty print this list of diagnostics.
    pub fn pretty_print(
        &self,
        f: &mut dyn std::fmt::Write,
        source_by_hash: &impl GetSourceByHash,
    ) -> std::fmt::Result {
        self.0
            .iter()
            .try_for_each(|diag| diag.pretty_print(f, source_by_hash))
    }
}

impl PushDiag for DiagList {
    fn push_diag(&mut self, diag: Diagnostic) -> DiagResult<()> {
        self.0.push(diag);
        Ok(())
    }
}
