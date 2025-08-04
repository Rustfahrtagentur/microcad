// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad syntax elements related to expressions

mod array_expression;
mod marker;
mod nested;
mod nested_item;
mod range_expression;
mod tuple_expression;

pub use array_expression::*;
pub use marker::*;
pub use nested::*;
pub use nested_item::*;
pub use range_expression::*;
pub use tuple_expression::*;

use crate::{src_ref::*, syntax::*, value::*};

/// List of expression
pub type ExpressionList = Vec<Expression>;

/// Expressions
#[derive(Clone, Debug, Default)]
pub enum Expression {
    /// Something went wrong (and an error will be reported)
    #[default]
    Invalid,
    /// A processed value.
    Value(Value),
    /// An integer, float, color or bool literal: 1, 1.0, #00FF00, false
    Literal(Literal),
    /// A string that contains format expressions: "value = {a}"
    FormatString(FormatString),
    /// A list: [a, b, c]
    ArrayExpression(ArrayExpression),
    /// A tuple: (a, b, c)
    TupleExpression(TupleExpression),
    /// A list whitespace separated of nested items: `translate() rotate()`, `b c`, `a b() {}`
    Nested(Nested),
    /// A marker expression: `@children`.
    Marker(Marker),
    /// A binary operation: `a + b`
    BinaryOp {
        /// Left-hand side
        lhs: Box<Expression>,
        /// Operator  ('+', '-', '/', '*', '<', '>', '≤', '≥', '&', '|')
        op: String,
        /// Right -hand side
        rhs: Box<Expression>,
        /// Source code reference
        src_ref: SrcRef,
    },
    /// A unary operation: !a
    UnaryOp {
        /// Operator ('+', '-', '!')
        op: String,
        /// Right -hand side
        rhs: Box<Expression>,
        /// Source code reference
        src_ref: SrcRef,
    },
    /// Access an element of a list (`a[0]`) or a tuple (`a.0` or `a.b`)
    ArrayElementAccess(Box<Expression>, Box<Expression>, SrcRef),
    /// Access an element of a tuple: `a.b`
    PropertyAccess(Box<Expression>, Identifier, SrcRef),

    /// Access an attribute of a model: `a#b`
    AttributeAccess(Box<Expression>, Identifier, SrcRef),

    /// Call to a method: `[2,3].len()`
    /// First expression must evaluate to a value
    MethodCall(Box<Expression>, MethodCall, SrcRef),
}

impl SrcReferrer for Expression {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        match self {
            Self::Invalid => SrcRef(None),
            Self::Value(_) => SrcRef(None),
            Self::Literal(l) => l.src_ref(),
            Self::FormatString(fs) => fs.src_ref(),
            Self::ArrayExpression(le) => le.src_ref(),
            Self::TupleExpression(te) => te.src_ref(),
            Self::Nested(n) => n.src_ref(),
            Self::Marker(m) => m.src_ref(),
            Self::BinaryOp {
                lhs: _,
                op: _,
                rhs: _,
                src_ref,
            } => src_ref.clone(),
            Self::UnaryOp {
                op: _,
                rhs: _,
                src_ref,
            } => src_ref.clone(),
            Self::ArrayElementAccess(_, _, src_ref) => src_ref.clone(),
            Self::PropertyAccess(_, _, src_ref) => src_ref.clone(),
            Self::AttributeAccess(_, _, src_ref) => src_ref.clone(),
            Self::MethodCall(_, _, src_ref) => src_ref.clone(),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Value(value) => write!(f, "{value}"),
            Self::Literal(literal) => write!(f, "{literal}"),
            Self::FormatString(format_string) => write!(f, "{format_string}"),
            Self::ArrayExpression(array_expression) => write!(f, "{array_expression}"),
            Self::TupleExpression(tuple_expression) => write!(f, "{tuple_expression}"),
            Self::BinaryOp {
                lhs,
                op,
                rhs,
                src_ref: _,
            } => write!(f, "{lhs} {op} {rhs}"),
            Self::UnaryOp {
                op,
                rhs,
                src_ref: _,
            } => write!(f, "{op}{rhs}"),
            Self::ArrayElementAccess(lhs, rhs, _) => write!(f, "{lhs}[{rhs}]"),
            Self::PropertyAccess(lhs, rhs, _) => write!(f, "{lhs}.{rhs}"),
            Self::AttributeAccess(lhs, rhs, _) => write!(f, "{lhs}#{rhs}"),
            Self::MethodCall(lhs, method_call, _) => write!(f, "{lhs}.{method_call}"),
            Self::Nested(nested) => write!(f, "{nested}"),
            Self::Marker(marker) => write!(f, "{marker}"),
            _ => unimplemented!(),
        }
    }
}

impl PrintSyntax for Value {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        write!(f, "{:depth$}Value: {value}", "", value = self)
    }
}

impl PrintSyntax for Expression {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        match self {
            Expression::Value(value) => value.print_syntax(f, depth),
            Expression::Literal(literal) => literal.print_syntax(f, depth),
            Expression::FormatString(format_string) => format_string.print_syntax(f, depth),
            Expression::ArrayExpression(array_expression) => {
                array_expression.print_syntax(f, depth)
            }
            Expression::TupleExpression(tuple_expression) => {
                tuple_expression.print_syntax(f, depth)
            }
            Expression::BinaryOp {
                lhs,
                op,
                rhs,
                src_ref: _,
            } => {
                writeln!(f, "{:depth$}BinaryOp '{op}':", "")?;
                lhs.print_syntax(f, depth + Self::INDENT)?;
                rhs.print_syntax(f, depth + Self::INDENT)
            }
            Expression::UnaryOp {
                op,
                rhs,
                src_ref: _,
            } => {
                writeln!(f, "{:depth$}UnaryOp '{op}':", "")?;
                rhs.print_syntax(f, depth + Self::INDENT)
            }
            Expression::ArrayElementAccess(lhs, rhs, _) => {
                writeln!(f, "{:depth$}ArrayElementAccess:", "")?;
                lhs.print_syntax(f, depth + Self::INDENT)?;
                rhs.print_syntax(f, depth + Self::INDENT)
            }
            Expression::PropertyAccess(lhs, rhs, _) => {
                writeln!(f, "{:depth$}FieldAccess:", "")?;
                lhs.print_syntax(f, depth + Self::INDENT)?;
                rhs.print_syntax(f, depth + Self::INDENT)
            }
            Expression::AttributeAccess(lhs, rhs, _) => {
                writeln!(f, "{:depth$}AttributeAccess:", "")?;
                lhs.print_syntax(f, depth + Self::INDENT)?;
                rhs.print_syntax(f, depth + Self::INDENT)
            }
            Expression::MethodCall(lhs, method_call, _) => {
                writeln!(f, "{:depth$}MethodCall:", "")?;
                lhs.print_syntax(f, depth + Self::INDENT)?;
                method_call.print_syntax(f, depth + Self::INDENT)
            }
            Expression::Nested(nested) => nested.print_syntax(f, depth),
            Expression::Marker(marker) => marker.print_syntax(f, depth),
            Expression::Invalid => write!(f, "{}", crate::invalid!(EXPRESSION)),
        }
    }
}

impl Expression {
    /// If the expression consists of a single identifier, e.g. `a`
    pub fn single_identifier(&self) -> Option<Identifier> {
        match &self {
            Self::Nested(nested) => nested.single_identifier(),
            _ => None,
        }
    }
}
