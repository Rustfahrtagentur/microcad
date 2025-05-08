// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module statement syntax elements
//!
use crate::{src_ref::*, syntax::*};

/// Module statement
#[derive(Clone, Debug)]
pub struct ReturnStatement {
    /// return value
    pub result: Option<Expression>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl SrcReferrer for ReturnStatement {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for ReturnStatement {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        todo!()
    }
}

impl PrintSyntax for ReturnStatement {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}ReturnStatement:", "")?;
        if let Some(result) = &self.result {
            result.print_syntax(f, depth + 1)?;
        }
        Ok(())
    }
}
