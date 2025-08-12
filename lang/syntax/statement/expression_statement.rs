// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Expression statement syntax elements

use crate::{src_ref::*, syntax::*};

/// An assignment statement, e.g. `#[aux] s = Sphere(3.0mm);`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExpressionStatement {
    /// Optional attributes.
    pub attribute_list: AttributeList,
    /// The actual expression.
    pub expression: Expression,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl SrcReferrer for ExpressionStatement {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl TreeDisplay for ExpressionStatement {
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result {
        self.expression.tree_print(f, depth)
    }
}

impl std::fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.attribute_list.is_empty() {
            write!(f, "{} ", self.attribute_list)?;
        }
        write!(f, "{};", self.expression)
    }
}
