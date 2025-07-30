// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad model assignment syntax element

use crate::{src_ref::*, syntax::*};

/// Model assignment specifying an identifier, type and value
#[derive(Clone, Debug)]
pub struct ModelAssignment {
    /// Assignee
    pub id: Identifier,
    /// Value to assign
    pub expression: ModelExpression,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl SrcReferrer for ModelAssignment {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for ModelAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} := {}", self.id, self.expression)
    }
}

impl PrintSyntax for ModelAssignment {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}ModelAssignment {}:", "", self.id)?;
        self.expression.print_syntax(f, depth + Self::INDENT)
    }
}
