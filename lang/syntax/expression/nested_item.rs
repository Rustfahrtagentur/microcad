// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Nested item syntax element

use crate::{src_ref::*, syntax::*};

/// Nested item
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum NestedItem {
    /// Call
    Call(Call),
    /// Qualified Name
    QualifiedName(QualifiedName),
    /// Object body
    Body(Body),
}

impl SrcReferrer for NestedItem {
    fn src_ref(&self) -> SrcRef {
        match self {
            Self::Call(c) => c.src_ref(),
            Self::QualifiedName(qn) => qn.src_ref(),
            Self::Body(nb) => nb.src_ref(),
        }
    }
}

impl std::fmt::Display for NestedItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Call(call) => write!(f, "{call}"),
            Self::QualifiedName(qualified_name) => write!(f, "{qualified_name}"),
            Self::Body(body) => write!(f, "{body}"),
        }
    }
}

impl TreeDisplay for NestedItem {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeIndent) -> std::fmt::Result {
        writeln!(f, "{:depth$}NestedItem:", "")?;
        depth.indent();
        match self {
            Self::Call(call) => call.tree_print(f, depth),
            Self::QualifiedName(qualified_name) => qualified_name.tree_print(f, depth),
            Self::Body(body) => body.tree_print(f, depth),
        }
    }
}
