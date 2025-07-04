// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A single argument

use crate::{src_ref::*, syntax::*};

/// Argument
#[derive(Clone, Debug)]
pub struct Argument {
    /// Name of the argument
    pub id: Option<Identifier>,
    /// Value of the argument
    pub value: Expression,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl SrcReferrer for Argument {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl PrintSyntax for Argument {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        match self.id {
            Some(ref name) => writeln!(f, "{:depth$}Argument '{}':", "", name)?,
            None => writeln!(f, "{:depth$}Argument:", "")?,
        };
        self.value.print_syntax(f, depth)
    }
}
