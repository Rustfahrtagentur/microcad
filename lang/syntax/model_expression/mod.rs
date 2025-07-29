// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad syntax elements related to expressions

mod marker;
mod nested;
mod nested_item;

pub use marker::*;
pub use nested::*;
pub use nested_item::*;

use crate::{src_ref::*, syntax::*};

/// List of expression
pub type ModelExpressionList = Vec<ModelExpression>;

/// Expressions
#[derive(Clone, Debug, Default)]
pub enum ModelExpression {
    /// Something went wrong (and an error will be reported)
    #[default]
    Invalid,
    /// A list whitespace separated of nested items: `translate() rotate()`, `b c`, `a b() {}`
    Nested(Nested),
    /// A marker expression: `@children`.
    Marker(Marker),
    /// A binary operation: `a + b`
    BinaryOp {
        /// Left-hand side
        lhs: Box<ModelExpression>,
        /// Operator  ('+', '-', '/', '*', '<', '>', '≤', '≥', '&', '|')
        op: String,
        /// Right -hand side
        rhs: Box<ModelExpression>,
        /// Source code reference
        src_ref: SrcRef,
    },
    /// Access an element of a tuple: `a.b`
    PropertyAccess(Box<ModelExpression>, Identifier, SrcRef),
    /// Access an attribute of a model: `a#b`
    AttributeAccess(Box<ModelExpression>, Identifier, SrcRef),
    /// Call to a method: `[2,3].len()`
    /// First expression must evaluate to a value
    MethodCall(Box<ModelExpression>, MethodCall, SrcRef),
}

impl SrcReferrer for ModelExpression {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        match self {
            Self::Invalid => SrcRef(None),
            Self::Nested(n) => n.src_ref(),
            Self::Marker(m) => m.src_ref(),
            Self::BinaryOp {
                lhs: _,
                op: _,
                rhs: _,
                src_ref,
            } => src_ref.clone(),
            Self::PropertyAccess(_, _, src_ref) => src_ref.clone(),
            Self::AttributeAccess(_, _, src_ref) => src_ref.clone(),
            Self::MethodCall(_, _, src_ref) => src_ref.clone(),
        }
    }
}

impl std::fmt::Display for ModelExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::BinaryOp {
                lhs,
                op,
                rhs,
                src_ref: _,
            } => write!(f, "{lhs} {op} {rhs}"),
            Self::PropertyAccess(lhs, rhs, _) => write!(f, "{lhs}.{rhs}"),
            Self::AttributeAccess(lhs, rhs, _) => write!(f, "{lhs}#{rhs}"),
            Self::MethodCall(lhs, method_call, _) => write!(f, "{lhs}.{method_call}"),
            Self::Nested(nested) => write!(f, "{nested}"),
            Self::Marker(marker) => write!(f, "{marker}"),
            _ => unimplemented!(),
        }
    }
}

impl PrintSyntax for ModelExpression {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        match self {
            Self::BinaryOp {
                lhs,
                op,
                rhs,
                src_ref: _,
            } => {
                writeln!(f, "{:depth$}BinaryOp '{op}':", "")?;
                lhs.print_syntax(f, depth + Self::INDENT)?;
                rhs.print_syntax(f, depth + Self::INDENT)
            }
            Self::PropertyAccess(lhs, rhs, _) => {
                writeln!(f, "{:depth$}FieldAccess:", "")?;
                lhs.print_syntax(f, depth + Self::INDENT)?;
                rhs.print_syntax(f, depth + Self::INDENT)
            }
            Self::AttributeAccess(lhs, rhs, _) => {
                writeln!(f, "{:depth$}AttributeAccess:", "")?;
                lhs.print_syntax(f, depth + Self::INDENT)?;
                rhs.print_syntax(f, depth + Self::INDENT)
            }
            Self::MethodCall(lhs, method_call, _) => {
                writeln!(f, "{:depth$}MethodCall:", "")?;
                lhs.print_syntax(f, depth + Self::INDENT)?;
                method_call.print_syntax(f, depth + Self::INDENT)
            }
            Self::Nested(nested) => nested.print_syntax(f, depth),
            Self::Marker(marker) => marker.print_syntax(f, depth),
            Self::Invalid => write!(f, "{}", crate::invalid!(MODEL_EXPRESSION)),
        }
    }
}

impl ModelExpression {
    /// If the expression consists of a single identifier, e.g. `a`
    pub fn single_identifier(&self) -> Option<Identifier> {
        match &self {
            Self::Nested(nested) => nested.single_identifier(),
            _ => None,
        }
    }
}
