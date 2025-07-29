// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Expression statement syntax elements

use crate::{src_ref::*, syntax::*};

/// An assignment statement, e.g. `#[aux] s = sphere(3.0mm);`.
#[derive(Debug, Clone)]
pub struct ModelExpressionStatement {
    /// Optional attributes.
    pub attribute_list: AttributeList,
    /// The actual expression.
    pub expression: ModelExpression,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl SrcReferrer for ModelExpressionStatement {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl PrintSyntax for ModelExpressionStatement {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        self.expression.print_syntax(f, depth)
    }
}

impl std::fmt::Display for ModelExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.attribute_list.is_empty() {
            write!(f, "{} ", self.attribute_list)?;
        }
        write!(f, "{};", self.expression)
    }
}
