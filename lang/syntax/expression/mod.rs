// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad syntax elements related to expressions

mod list_expression;
mod nested;
mod nested_item;
mod tuple_expression;

pub use list_expression::*;
pub use nested::*;
pub use nested_item::*;
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
    /// A processed value, a result from calling lower()
    Value(Value),
    /// An integer, float, color or bool literal: 1, 1.0, #00FF00, false
    Literal(Literal),
    /// A string that contains format expressions: "value = {a}"
    FormatString(FormatString),
    /// A list: [a, b, c]
    ListExpression(ListExpression),
    /// A tuple: (a, b, c)
    TupleExpression(TupleExpression),
    /// A list whitespace separated of nested items: `translate() rotate()`, `b c`, `a b() {}`
    Nested(Nested),
    /// A binary operation: a + b
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

    /// Access an attribute of a node: `a#b`
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
            Self::ListExpression(le) => le.src_ref(),
            Self::TupleExpression(te) => te.src_ref(),
            Self::Nested(n) => n.src_ref().clone(),
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
            Self::ListExpression(list_expression) => write!(f, "{list_expression}"),
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
        let depth = depth + 1;
        match self {
            Self::Value(value) => value.print_syntax(f, depth),
            Self::Literal(literal) => literal.print_syntax(f, depth),
            Self::FormatString(format_string) => format_string.print_syntax(f, depth),
            Self::ListExpression(list_expression) => list_expression.print_syntax(f, depth),
            Self::TupleExpression(tuple_expression) => tuple_expression.print_syntax(f, depth),
            Self::BinaryOp {
                lhs,
                op,
                rhs,
                src_ref: _,
            } => {
                writeln!(f, "{:depth$}BinaryOp '{op}':", "")?;
                lhs.print_syntax(f, depth)?;
                rhs.print_syntax(f, depth)
            }
            Self::UnaryOp {
                op,
                rhs,
                src_ref: _,
            } => {
                writeln!(f, "{:depth$}UnaryOp '{op}':", "")?;
                rhs.print_syntax(f, depth)
            }
            Self::ArrayElementAccess(lhs, rhs, _) => {
                writeln!(f, "{:depth$}ArrayElementAccess:", "")?;
                lhs.print_syntax(f, depth)?;
                rhs.print_syntax(f, depth)
            }
            Self::PropertyAccess(lhs, rhs, _) => {
                writeln!(f, "{:depth$}FieldAccess:", "")?;
                lhs.print_syntax(f, depth)?;
                rhs.print_syntax(f, depth)
            }
            Self::AttributeAccess(lhs, rhs, _) => {
                writeln!(f, "{:depth$}AttributeAccess:", "")?;
                lhs.print_syntax(f, depth)?;
                rhs.print_syntax(f, depth)
            }
            Self::MethodCall(lhs, method_call, _) => {
                writeln!(f, "{:depth$}MethodCall:", "")?;
                lhs.print_syntax(f, depth)?;
                method_call.print_syntax(f, depth)
            }
            Self::Nested(nested) => nested.print_syntax(f, depth),
            _ => unimplemented!(),
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
