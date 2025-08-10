// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! If statement syntax elements.

use crate::{src_ref::*, syntax::*};

/// If statement.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IfStatement {
    /// If condition.
    pub cond: Expression,
    /// Body if `true`.
    pub body: Body,
    /// Body if `false`.
    pub body_else: Option<Body>,
    /// Next if statement: `else if x == 1`.
    pub next_if: Option<Box<IfStatement>>,
    /// Source code reference.
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

impl TreeDisplay for IfStatement {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}IfStatement:", "")?;
        depth.indent();
        writeln!(f, "{:depth$}Condition:", "")?;
        self.cond.tree_print(f, depth.indented())?;
        writeln!(f, "{:depth$}If:", "")?;
        self.body.tree_print(f, depth.indented())?;
        if let Some(body_else) = &self.body_else {
            writeln!(f, "{:depth$}Else:", "")?;
            body_else.tree_print(f, depth.indented())?;
        }
        Ok(())
    }
}
