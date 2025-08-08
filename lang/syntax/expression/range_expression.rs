// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Range expression

use crate::{
    src_ref::{SrcRef, SrcReferrer},
    syntax::{Expression, PrintSyntax},
};

/// Range start.
#[derive(Clone, Debug, Default)]
pub struct RangeStart(pub Box<Expression>);

impl SrcReferrer for RangeStart {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.0.src_ref()
    }
}

impl std::fmt::Display for RangeStart {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl PrintSyntax for RangeStart {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}RangeStart:", "")?;
        let depth = depth + Self::INDENT;
        self.0.print_syntax(f, depth)
    }
}

/// Range end.
#[derive(Clone, Debug, Default)]
pub struct RangeEnd(pub Box<Expression>);

impl SrcReferrer for RangeEnd {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.0.src_ref()
    }
}

impl std::fmt::Display for RangeEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl PrintSyntax for RangeEnd {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}RangeEnd:", "")?;
        let depth = depth + Self::INDENT;
        self.0.print_syntax(f, depth)
    }
}

/// Range expression: `a..b`.
#[derive(Clone, Debug, Default)]
pub struct RangeExpression {
    /// Start of the range.
    pub start: RangeStart,
    /// End of the range.
    pub end: RangeEnd,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl SrcReferrer for RangeExpression {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for RangeExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}
impl PrintSyntax for RangeExpression {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}RangeExpression:", "")?;
        let depth = depth + Self::INDENT;
        self.start.print_syntax(f, depth)?;
        self.end.print_syntax(f, depth)
    }
}
