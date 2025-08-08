// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Format expression syntax element

use crate::{src_ref::*, syntax::*};

/// Format expression including format specification
#[allow(dead_code)]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FormatExpression {
    /// Format specifier
    pub spec: Option<FormatSpec>,
    /// Expression to format
    pub expression: Expression,
    /// Source code reference
    src_ref: SrcRef,
}

impl FormatExpression {
    /// Create new format expression
    pub fn new(spec: Option<FormatSpec>, expression: Expression, src_ref: SrcRef) -> Self {
        Self {
            src_ref,
            spec,
            expression,
        }
    }
}

impl std::fmt::Display for FormatExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(spec) = &self.spec {
            write!(f, "{{{}:{}}}", spec, self.expression)
        } else {
            write!(f, "{{{}}}", self.expression)
        }
    }
}

impl SrcReferrer for FormatExpression {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl TreeDisplay for FormatExpression {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeIndent) -> std::fmt::Result {
        writeln!(f, "{:depth$}FormatExpression:", "")?;
        depth.indent();
        if let Some(spec) = &self.spec {
            spec.tree_print(f, depth)?;
            self.expression.tree_print(f, depth)
        } else {
            self.expression.tree_print(f, depth)
        }
    }
}
