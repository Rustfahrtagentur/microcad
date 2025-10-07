// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad syntax elements related to expressions

mod array_expression;
mod marker;
mod range_expression;
mod tuple_expression;

pub use array_expression::*;
pub use marker::*;
pub use range_expression::*;
pub use tuple_expression::*;

use crate::{src_ref::*, syntax::*, value::*};

/// List of expressions.
pub type ListExpression = Vec<Expression>;

pub(crate) trait Const {
    fn is_const(&self) -> bool;
}

/// Any expression.
#[derive(Clone, Default)]
pub enum Expression {
    /// Something went wrong (and an error will be reported)
    #[default]
    Invalid,
    /// An integer, float, color or bool literal: 1, 1.0, #00FF00, false
    Literal(Literal),
    /// A string that contains format expressions: "value = {a}"
    FormatString(FormatString),
    /// A list: [a, b, c]
    ArrayExpression(ArrayExpression),
    /// A tuple: (a, b, c)
    TupleExpression(TupleExpression),
    /// A body: `{}`.
    Body(Body),
    /// A call: `ops::subtract()`.
    Call(Call),
    /// A qualified name: `foo::bar`.
    QualifiedName(QualifiedName),
    /// A marker expression: `@input`.
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
    /// Access an element of a tuple: `a.b`.
    PropertyAccess(Box<Expression>, Identifier, SrcRef),

    /// Access an attribute of a model: `a#b`.
    AttributeAccess(Box<Expression>, Identifier, SrcRef),

    /// Call to a method: `[2,3].len()`
    /// First expression must evaluate to a value
    MethodCall(Box<Expression>, MethodCall, SrcRef),
}

impl Expression {
    /// If the expression consists of a single identifier, e.g. `a`
    pub fn single_identifier(&self) -> Option<&Identifier> {
        match &self {
            Self::QualifiedName(qualified_name) => qualified_name.single_identifier(),
            _ => None,
        }
    }
}

impl SrcReferrer for Expression {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        match self {
            Self::Invalid => SrcRef(None),
            Self::Literal(l) => l.src_ref(),
            Self::FormatString(fs) => fs.src_ref(),
            Self::ArrayExpression(le) => le.src_ref(),
            Self::TupleExpression(te) => te.src_ref(),
            Self::Call(c) => c.src_ref(),
            Self::Body(b) => b.src_ref(),
            Self::QualifiedName(q) => q.src_ref(),
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
            Self::Call(call) => write!(f, "{call}"),
            Self::Body(body) => write!(f, "{body}"),
            Self::QualifiedName(qualified_name) => write!(f, "{qualified_name}"),
            Self::Marker(marker) => write!(f, "{marker}"),
            _ => unimplemented!(),
        }
    }
}

impl std::fmt::Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{literal}"),
            Self::FormatString(format_string) => write!(f, "{format_string:?}"),
            Self::ArrayExpression(array_expression) => write!(f, "{array_expression:?}"),
            Self::TupleExpression(tuple_expression) => write!(f, "{tuple_expression:?}"),
            Self::BinaryOp {
                lhs,
                op,
                rhs,
                src_ref: _,
            } => write!(f, "{lhs:?} {op} {rhs:?}"),
            Self::UnaryOp {
                op,
                rhs,
                src_ref: _,
            } => write!(f, "{op}{rhs:?}"),
            Self::ArrayElementAccess(lhs, rhs, _) => write!(f, "{lhs:?}[{rhs:?}]"),
            Self::PropertyAccess(lhs, rhs, _) => write!(f, "{lhs:?}.{rhs:?}"),
            Self::AttributeAccess(lhs, rhs, _) => write!(f, "{lhs:?}#{rhs:?}"),
            Self::MethodCall(lhs, method_call, _) => write!(f, "{lhs:?}.{method_call:?}"),
            Self::Call(call) => write!(f, "{call:?}"),
            Self::Body(body) => write!(f, "{body:?}"),
            Self::QualifiedName(qualified_name) => write!(f, "{qualified_name:?}"),
            Self::Marker(marker) => write!(f, "{marker:?}"),
            _ => unimplemented!(),
        }
    }
}

impl TreeDisplay for Value {
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result {
        write!(f, "{:depth$}Value: {value}", "", value = self)
    }
}

impl TreeDisplay for Expression {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        match self {
            Expression::Literal(literal) => literal.tree_print(f, depth),
            Expression::FormatString(format_string) => format_string.tree_print(f, depth),
            Expression::ArrayExpression(array_expression) => array_expression.tree_print(f, depth),
            Expression::TupleExpression(tuple_expression) => tuple_expression.tree_print(f, depth),
            Expression::BinaryOp {
                lhs,
                op,
                rhs,
                src_ref: _,
            } => {
                writeln!(f, "{:depth$}BinaryOp '{op}':", "")?;
                depth.indent();
                lhs.tree_print(f, depth)?;
                rhs.tree_print(f, depth)
            }
            Expression::UnaryOp {
                op,
                rhs,
                src_ref: _,
            } => {
                writeln!(f, "{:depth$}UnaryOp '{op}':", "")?;
                depth.indent();
                rhs.tree_print(f, depth)
            }
            Expression::ArrayElementAccess(lhs, rhs, _) => {
                writeln!(f, "{:depth$}ArrayElementAccess:", "")?;
                depth.indent();
                lhs.tree_print(f, depth)?;
                rhs.tree_print(f, depth)
            }
            Expression::PropertyAccess(lhs, rhs, _) => {
                writeln!(f, "{:depth$}FieldAccess:", "")?;
                depth.indent();
                lhs.tree_print(f, depth)?;
                rhs.tree_print(f, depth)
            }
            Expression::AttributeAccess(lhs, rhs, _) => {
                writeln!(f, "{:depth$}AttributeAccess:", "")?;
                depth.indent();
                lhs.tree_print(f, depth)?;
                rhs.tree_print(f, depth)
            }
            Expression::MethodCall(lhs, method_call, _) => {
                writeln!(f, "{:depth$}MethodCall:", "")?;
                depth.indent();
                lhs.tree_print(f, depth)?;
                method_call.tree_print(f, depth)
            }
            Expression::Call(call) => call.tree_print(f, depth),
            Expression::Body(body) => body.tree_print(f, depth),
            Expression::QualifiedName(qualified_name) => qualified_name.tree_print(f, depth),
            Expression::Marker(marker) => marker.tree_print(f, depth),
            Expression::Invalid => write!(f, "{}", crate::invalid!(EXPRESSION)),
        }
    }
}

impl AsRef<Self> for Expression {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Const for Expression {
    fn is_const(&self) -> bool {
        match self {
            Expression::Literal(_) => true,
            Expression::ArrayExpression(array_expression) => array_expression.is_const(),
            Expression::TupleExpression(tuple_expression) => tuple_expression.is_const(),
            Expression::QualifiedName(_) => {
                // in real this can only be determined after looking up the name
                true
            }
            Expression::BinaryOp {
                lhs, op: _, rhs, ..
            } => lhs.is_const() && rhs.is_const(),
            Expression::UnaryOp { op: _, rhs, .. } => rhs.is_const(),
            Expression::ArrayElementAccess(expression, expression1, ..) => {
                expression.is_const() && expression1.is_const()
            }
            Expression::AttributeAccess(expression, ..) => expression.is_const(),
            _ => false,
        }
    }
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Literal(l0), Self::Literal(r0)) => l0 == r0,
            (Self::FormatString(l0), Self::FormatString(r0)) => l0 == r0,
            (Self::ArrayExpression(l0), Self::ArrayExpression(r0)) => l0 == r0,
            (Self::TupleExpression(l0), Self::TupleExpression(r0)) => l0 == r0,
            (Self::QualifiedName(l0), Self::QualifiedName(r0)) => l0 == r0,
            (
                Self::BinaryOp {
                    lhs: l_lhs,
                    op: l_op,
                    rhs: l_rhs,
                    src_ref: l_src_ref,
                },
                Self::BinaryOp {
                    lhs: r_lhs,
                    op: r_op,
                    rhs: r_rhs,
                    src_ref: r_src_ref,
                },
            ) => l_lhs == r_lhs && l_op == r_op && l_rhs == r_rhs && l_src_ref == r_src_ref,
            (
                Self::UnaryOp {
                    op: l_op,
                    rhs: l_rhs,
                    src_ref: l_src_ref,
                },
                Self::UnaryOp {
                    op: r_op,
                    rhs: r_rhs,
                    src_ref: r_src_ref,
                },
            ) => l_op == r_op && l_rhs == r_rhs && l_src_ref == r_src_ref,
            (Self::ArrayElementAccess(l0, l1, l2), Self::ArrayElementAccess(r0, r1, r2)) => {
                l0 == r0 && l1 == r1 && l2 == r2
            }
            _ => unreachable!("PartialEq implemented for const expressions only"),
        }
    }
}
