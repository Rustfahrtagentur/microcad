// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model assignment statement syntax elements

use crate::{src_ref::*, syntax::*};

/// An assignment statement, e.g. `#[aux] s = sphere(3.0mm);`.
#[derive(Clone, Debug)]
pub struct ModelAssignmentStatement {
    /// List of attributes.
    pub attribute_list: AttributeList,
    /// The actual assignment.
    pub assignment: ModelAssignment,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl SrcReferrer for ModelAssignmentStatement {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl PrintSyntax for ModelAssignmentStatement {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}ModelAssignment '{}'", "", self.assignment)
    }
}

impl std::fmt::Display for ModelAssignmentStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.attribute_list.is_empty() {
            write!(f, "{} ", self.attribute_list)?;
        }
        write!(f, "{};", self.assignment)
    }
}
