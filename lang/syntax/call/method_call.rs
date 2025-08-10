// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Method call syntax elements.

use crate::{src_ref::*, syntax::*};

/// Method call syntax entity.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MethodCall {
    /// Name of the method.
    pub id: QualifiedName,
    /// List of arguments.
    pub argument_list: ArgumentList,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl SrcReferrer for MethodCall {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for MethodCall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({})", self.id, self.argument_list)
    }
}

impl TreeDisplay for MethodCall {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}MethodCall '{}':", "", self.id)?;
        depth.indent();
        self.argument_list.tree_print(f, depth)
    }
}
