// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Nested item syntax element

use crate::{src_ref::*, syntax::*};

/// Nested item
#[derive(Clone, Debug)]
pub enum NestedItem {
    /// Call
    Call(Call),
    /// Qualified Name
    QualifiedName(QualifiedName),
    /// Object body
    Body(Body),
    /// Marker
    Marker(Marker),
}

impl SrcReferrer for NestedItem {
    fn src_ref(&self) -> SrcRef {
        match self {
            Self::Call(c) => c.src_ref(),
            Self::QualifiedName(qn) => qn.src_ref(),
            Self::Body(nb) => nb.src_ref(),
            Self::Marker(m) => m.src_ref(),
        }
    }
}

impl std::fmt::Display for NestedItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Call(call) => write!(f, "{call}"),
            Self::QualifiedName(qualified_name) => write!(f, "{qualified_name}"),
            Self::Body(body) => write!(f, "{body}"),
            Self::Marker(marker) => write!(f, "{marker}"),
        }
    }
}

impl PrintSyntax for NestedItem {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}NestedItem:", "")?;
        match self {
            Self::Call(call) => call.print_syntax(f, depth + Self::INDENT),
            Self::QualifiedName(qualified_name) => {
                qualified_name.print_syntax(f, depth + Self::INDENT)
            }
            Self::Body(body) => body.print_syntax(f, depth + Self::INDENT),
            Self::Marker(marker) => marker.print_syntax(f, depth + Self::INDENT),
        }
    }
}
