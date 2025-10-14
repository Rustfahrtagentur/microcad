// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! List of expression

use crate::{
    src_ref::*,
    syntax::{expression::range_expression::RangeExpression, *},
};
use derive_more::{Deref, DerefMut};

/// Inner of an [`ArrayExpression`].
#[derive(Clone, PartialEq)]
pub enum ArrayExpressionInner {
    /// List: `a,b,c`.
    List(ListExpression),
    /// Range: `a..b`.
    Range(RangeExpression),
}

impl Default for ArrayExpressionInner {
    fn default() -> Self {
        Self::List(Default::default())
    }
}

impl std::fmt::Display for ArrayExpressionInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                ArrayExpressionInner::List(expressions) => expressions
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
                ArrayExpressionInner::Range(range_expression) => range_expression.to_string(),
            }
        )
    }
}

impl std::fmt::Debug for ArrayExpressionInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                ArrayExpressionInner::List(expressions) => expressions
                    .iter()
                    .map(|c| format!("{c:?}"))
                    .collect::<Vec<_>>()
                    .join(", "),
                ArrayExpressionInner::Range(range_expression) => range_expression.to_string(),
            }
        )
    }
}

impl SrcReferrer for ArrayExpressionInner {
    fn src_ref(&self) -> SrcRef {
        match &self {
            ArrayExpressionInner::List(expressions) => SrcRef::merge(
                &expressions
                    .first()
                    .map(|start| start.src_ref())
                    .unwrap_or_default(),
                &expressions
                    .last()
                    .map(|end| end.src_ref())
                    .unwrap_or_default(),
            ),
            ArrayExpressionInner::Range(range_expression) => range_expression.src_ref(),
        }
    }
}

impl TreeDisplay for ArrayExpressionInner {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        match &self {
            ArrayExpressionInner::List(expressions) => {
                writeln!(f, "{:depth$}List:", "")?;
                depth.indent();
                expressions
                    .iter()
                    .try_for_each(|expression| expression.tree_print(f, depth))
            }
            ArrayExpressionInner::Range(range_expression) => range_expression.tree_print(f, depth),
        }
    }
}

/// Array of expressions with common result unit, e.g. `[1+2,4,9]`.
#[derive(Default, Clone, Deref, DerefMut, PartialEq)]
pub struct ArrayExpression {
    /// Expression list.
    #[deref]
    #[deref_mut]
    pub inner: ArrayExpressionInner,
    /// Unit.
    pub unit: Unit,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl SrcReferrer for ArrayExpression {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for ArrayExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]{}", self.inner, self.unit)
    }
}

impl std::fmt::Debug for ArrayExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{:?}]{}", self.inner, self.unit)
    }
}

impl TreeDisplay for ArrayExpression {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}ArrayExpression:", "")?;
        depth.indent();
        self.inner.tree_print(f, depth)?;
        self.unit.tree_print(f, depth)
    }
}
