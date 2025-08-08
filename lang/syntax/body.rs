// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Body syntax element.

use derive_more::Deref;

use crate::{resolve::*, src_ref::*, syntax::*};

/// A body is a list of statements inside `{}` brackets.
#[derive(Clone, Debug, Default, Deref, serde::Serialize, serde::Deserialize)]
pub struct Body {
    /// Body statements.
    #[deref]
    pub statements: StatementList,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl Body {
    /// fetches all symbols from the statements in the body.
    pub fn resolve(&self, parent: Option<Symbol>) -> SymbolMap {
        self.statements.fetch_symbol_map(parent)
    }
}

impl SrcReferrer for Body {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, " {{")?;
        writeln!(f, "{}", self.statements)?;
        writeln!(f, "}}")?;
        Ok(())
    }
}

impl TreeDisplay for Body {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeIndent) -> std::fmt::Result {
        writeln!(f, "{:depth$}Body:", "")?;
        depth.indent();
        self.statements
            .iter()
            .try_for_each(|s| s.tree_print(f, depth))
    }
}
