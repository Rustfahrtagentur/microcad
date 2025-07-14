// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad assignment syntax element

use crate::{src_ref::*, syntax::*, ty::*};

/// Assignment specifying an identifier, type and value
#[derive(Clone, Debug)]
pub struct Assignment {
    /// Assignee
    pub id: Identifier,
    /// Type of the assignee
    pub specified_type: Option<TypeAnnotation>,
    /// Value to assign
    pub expression: Expression,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl SrcReferrer for Assignment {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.specified_type {
            Some(t) => write!(f, "{}: {} = {}", self.id, t.ty(), self.expression),
            None => write!(f, "{} = {}", self.id, self.expression),
        }
    }
}

impl PrintSyntax for Assignment {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}Assignment {}:", "", self.id)?;
        if let Some(specified_type) = &self.specified_type {
            specified_type.print_syntax(f, depth + Self::INDENT)?;
        }
        self.expression.print_syntax(f, depth + Self::INDENT)
    }
}
