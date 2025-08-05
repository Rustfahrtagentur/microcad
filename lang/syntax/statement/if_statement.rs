// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! If statement syntax elements
//!
use crate::{src_ref::*, syntax::*};

/// If statement
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IfStatement {
    /// if condition
    pub cond: Expression,
    /// body if true
    pub body: Body,
    /// body if false
    pub body_else: Option<Body>,
    /// next if statement: `else if x == 1`
    pub next_if: Option<Box<IfStatement>>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl SrcReferrer for IfStatement {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for IfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "if {cond} {body}", cond = self.cond, body = self.body)?;
        if let Some(body) = &self.body_else {
            writeln!(f, "else {body}")?;
        }
        Ok(())
    }
}

impl PrintSyntax for IfStatement {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}IfStatement:", "")?;
        let depth = depth + Self::INDENT;
        writeln!(f, "{:depth$}Condition:", "")?;
        self.cond.print_syntax(f, depth + Self::INDENT)?;
        self.body.print_syntax(f, depth + Self::INDENT)?;
        if let Some(body_else) = &self.body_else {
            writeln!(f, "{:depth$}Else:", "")?;
            body_else.print_syntax(f, depth + Self::INDENT)?;
        }
        Ok(())
    }
}
