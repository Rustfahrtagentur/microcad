// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Range expression

use derive_more::Deref;

use crate::{src_ref::*, syntax::*};

/// Range start.
#[derive(Clone, Default, Deref, PartialEq)]
pub struct RangeFirst(pub Box<Expression>);

impl SrcReferrer for RangeFirst {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.0.src_ref()
    }
}

impl std::fmt::Display for RangeFirst {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for RangeFirst {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl TreeDisplay for RangeFirst {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}RangeStart:", "")?;
        depth.indent();
        self.0.tree_print(f, depth)
    }
}

/// Range end.
#[derive(Clone, Default, Deref, PartialEq)]
pub struct RangeLast(pub Box<Expression>);

impl SrcReferrer for RangeLast {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.0.src_ref()
    }
}

impl std::fmt::Display for RangeLast {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for RangeLast {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl TreeDisplay for RangeLast {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}RangeLast:", "")?;
        depth.indent();
        self.0.tree_print(f, depth)
    }
}

/// Range expression, e.g. `a..b`.
#[derive(Clone, Default, PartialEq)]
pub struct RangeExpression {
    /// First value in the range.
    pub first: RangeFirst,
    /// Last value in the range.
    pub last: RangeLast,
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
        write!(f, "{}..{}", self.first, self.last)
    }
}

impl std::fmt::Debug for RangeExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}..{:?}", self.first, self.last)
    }
}

impl TreeDisplay for RangeExpression {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}RangeExpression:", "")?;
        depth.indent();
        self.first.tree_print(f, depth)?;
        self.last.tree_print(f, depth)
    }
}
