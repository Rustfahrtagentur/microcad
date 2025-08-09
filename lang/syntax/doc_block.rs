// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Documentation block syntax element

use crate::{src_ref::*, syntax::*};

/// Block of documentation comments, starting with `/// `.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DocBlock {
    /// Doc comment lines.
    pub lines: Vec<String>,
    /// Source reference.
    pub src_ref: SrcRef,
}

impl SrcReferrer for DocBlock {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for DocBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.lines
            .iter()
            .try_for_each(|doc| writeln!(f, "/// {doc}"))
    }
}

impl TreeDisplay for DocBlock {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeIndent) -> std::fmt::Result {
        writeln!(f, "{:depth$}DocBlock:", "")?;
        depth.indent();
        self.lines
            .iter()
            .try_for_each(|doc| writeln!(f, "{:depth$}/// {doc}", ""))
    }
}
