// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Range expression

use crate::{src_ref::*, syntax::*};

/// Range start.
#[derive(
    Clone, Debug, Default, bincode::Encode, bincode::Decode, serde::Serialize, serde::Deserialize,
)]
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

impl TreeDisplay for RangeStart {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}RangeStart:", "")?;
        depth.indent();
        self.0.tree_print(f, depth)
    }
}

/// Range end.
#[derive(
    Clone, Debug, Default, bincode::Encode, bincode::Decode, serde::Serialize, serde::Deserialize,
)]
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
impl TreeDisplay for RangeEnd {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}RangeEnd:", "")?;
        depth.indent();
        self.0.tree_print(f, depth)
    }
}

/// Range expression, e.g. `a..b`.
#[derive(
    Clone, Debug, Default, bincode::Encode, bincode::Decode, serde::Serialize, serde::Deserialize,
)]
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
impl TreeDisplay for RangeExpression {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}RangeExpression:", "")?;
        depth.indent();
        self.start.tree_print(f, depth)?;
        self.end.tree_print(f, depth)
    }
}
