// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! If statement parser entities
//!
use crate::{parse::*, parser::*, src_ref::*};

/// If statement
#[derive(Clone, Debug)]
pub struct IfStatement {
    /// if condition
    pub cond: Expression,
    /// body if true
    pub body: Body,
    /// body if false
    pub body_else: Option<Body>,
    /// Source code reference
    src_ref: SrcRef,
}

impl SrcReferrer for IfStatement {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Parse for IfStatement {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut cond = Default::default();
        let mut body = Body::default();
        let mut body_else = None;

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::expression => cond = Expression::parse(pair)?,
                Rule::body => body = Body::parse(pair)?,
                Rule::body_else => {
                    body_else = Some(Body::parse(pair.clone())?);
                }
                rule => unreachable!("Unexpected rule in if, got {:?}", rule),
            }
        }

        Ok(IfStatement {
            cond,
            body,
            body_else,
            src_ref: pair.into(),
        })
    }
}

impl std::fmt::Display for IfStatement {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        todo!()
    }
}

impl Syntax for IfStatement {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}ReturnStatement:", "")?;
        writeln!(f, "{:depth$}  Condition:", "")?;
        self.cond.print_syntax(f, depth + 1)?;
        writeln!(f, "{:depth$}  Body:", "")?;
        self.body.print_syntax(f, depth + 1)?;
        if let Some(body_else) = &self.body_else {
            writeln!(f, "{:depth$}  BodyElse:", "")?;
            body_else.print_syntax(f, depth + 1)?;
        }
        Ok(())
    }
}
