// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Return statement syntax elements

use crate::{src_ref::*, syntax::*};

/// Return statement
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(result) = &self.result {
            write!(f, "{result}")
        } else {
            write!(f, crate::invalid!(RESULT))
        }
    }
}

impl PrintSyntax for ReturnStatement {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}ReturnStatement:", "")?;
        if let Some(result) = &self.result {
            result.print_syntax(f, depth + Self::INDENT)?;
        }
        Ok(())
    }
}
