// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad parser entities related to expressions

mod expression_list;
mod list_expression;
mod nested;
mod nested_item;
mod tuple_expression;

pub use expression_list::*;
pub use list_expression::*;
pub use nested::*;
pub use nested_item::*;
pub use tuple_expression::*;

use crate::{parse::*, parser::*, src_ref::*};

lazy_static::lazy_static! {
    /// Expression parser
    static ref PRATT_PARSER: pest::pratt_parser::PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc, Op,PrattParser};
        use Assoc::*;
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
            .op(Op::infix(r#union, Left) | Op::infix(intersection, Left))
            .op(Op::infix(power_xor, Left))
            .op(Op::infix(greater_than, Left) | Op::infix(less_than, Left))
            .op(Op::infix(less_equal, Left) | Op::infix(greater_equal, Left))
            .op(Op::infix(equal, Left) | Op::infix(not_equal, Left))
            .op(Op::infix(near, Left))
            .op(Op::prefix(unary_minus))
            .op(Op::prefix(unary_plus))
            .op(Op::prefix(unary_not))
            .op(Op::postfix(method_call))
            .op(Op::postfix(list_element_access))
            .op(Op::postfix(tuple_element_access))
    };
}

/// Expressions
#[derive(Clone, Debug, Default)]
pub enum Expression {
    /// Something went wrong (and an error will be reported)
    #[default]
    Invalid,
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
    ListElementAccess(Box<Expression>, Box<Expression>, SrcRef),
    /// Access an element of a named tuple: `a.b`
    NamedTupleElementAccess(Box<Expression>, Identifier, SrcRef),
    /// Access an element of an unnamed tuple: `a.0`
    UnnamedTupleElementAccess(Box<Expression>, u32, SrcRef),
    /// Call to a method: `[2,3].len()`
    /// First expression must evaluate to a value
    MethodCall(Box<Expression>, MethodCall, SrcRef),
}

impl SrcReferrer for Expression {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        match self {
            Self::Invalid => SrcRef(None),
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
            Self::ListElementAccess(_, _, src_ref) => src_ref.clone(),
            Self::NamedTupleElementAccess(_, _, src_ref) => src_ref.clone(),
            Self::UnnamedTupleElementAccess(_, _, src_ref) => src_ref.clone(),
            Self::MethodCall(_, _, src_ref) => src_ref.clone(),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{literal}"),
            Self::FormatString(format_string) => write!(f, "{format_string}"),
            Self::ListExpression(list_expression) => write!(f, "{list_expression}"),
            Self::TupleExpression(tuple_expression) => write!(f, "{tuple_expression}"),
            Self::BinaryOp {
                lhs,
                op,
                rhs,
                src_ref: _,
            } => write!(f, "({lhs} {op} {rhs})"),
            Self::UnaryOp {
                op,
                rhs,
                src_ref: _,
            } => write!(f, "({op}{rhs})"),
            Self::ListElementAccess(lhs, rhs, _) => write!(f, "{lhs}[{rhs}]"),
            Self::NamedTupleElementAccess(lhs, rhs, _) => write!(f, "{lhs}.{rhs}"),
            Self::UnnamedTupleElementAccess(lhs, rhs, _) => write!(f, "{lhs}.{rhs}"),
            Self::MethodCall(lhs, method_call, _) => write!(f, "{lhs}.{method_call}"),
            Self::Nested(nested) => write!(f, "{nested}"),
            _ => unimplemented!(),
        }
    }
}

impl Expression {
    /// Generate literal from string
    pub fn literal_from_str(s: &str) -> ParseResult<Self> {
        use std::str::FromStr;
        if s.len() > 1 && s.starts_with('"') && s.ends_with('"') {
            Ok(Self::FormatString(FormatString::from_str(s)?))
        } else {
            Ok(Self::Literal(Literal::from_str(s)?))
        }
    }

    /// If the expression consists of a single identifier, e.g. `a`
    pub fn single_identifier(&self) -> Option<Identifier> {
        match &self {
            Self::Nested(nested) => nested.single_identifier(),
            _ => None,
        }
    }
}

impl Parse for Expression {
    fn parse(pair: Pair) -> ParseResult<Self> {
        if pair.as_rule() == Rule::expression_no_semicolon {
            return Ok(Self::Nested(Nested::parse(pair)?));
        }

        PRATT_PARSER
            .map_primary(|primary| {
                match (
                    Pair::new(primary.clone(), pair.source_hash()),
                    primary.as_rule(),
                ) {
                    (primary, Rule::literal) => Ok(Self::Literal(Literal::parse(primary)?)),
                    (primary, Rule::expression) => Ok(Self::parse(primary)?),
                    (primary, Rule::list_expression) => {
                        Ok(Self::ListExpression(ListExpression::parse(primary)?))
                    }
                    (primary, Rule::tuple_expression) => {
                        Ok(Self::TupleExpression(TupleExpression::parse(primary)?))
                    }
                    (primary, Rule::format_string) => {
                        Ok(Self::FormatString(FormatString::parse(primary)?))
                    }
                    (primary, Rule::nested) => Ok(Self::Nested(Nested::parse(primary)?)),
                    rule => unreachable!(
                        "Expression::parse expected atom, found {:?} {:?}",
                        rule,
                        pair.as_span().as_str()
                    ),
                }
            })
            .map_infix(|lhs, op, rhs| {
                let op = match op.as_rule() {
                    Rule::add => "+",
                    Rule::subtract => "-",
                    Rule::multiply => "*",
                    Rule::divide => "/",
                    Rule::r#union => "|",
                    Rule::intersection => "&",
                    Rule::power_xor => "^",
                    Rule::greater_than => ">",
                    Rule::less_than => "<",
                    Rule::less_equal => "≤",
                    Rule::greater_equal => "≥",
                    Rule::equal => "=",
                    Rule::near => "~",
                    Rule::not_equal => "!=",
                    Rule::and => "&",

                    rule => unreachable!(
                        "Expression::parse expected infix operation, found {:?}",
                        rule
                    ),
                };
                Ok(Self::BinaryOp {
                    lhs: Box::new(lhs?),
                    op: op.into(),
                    rhs: Box::new(rhs?),
                    src_ref: pair.clone().into(),
                })
            })
            .map_prefix(|op, rhs| {
                let op = match op.as_rule() {
                    Rule::unary_minus => '-',
                    Rule::unary_plus => '+',
                    Rule::unary_not => '!',
                    _ => unreachable!(),
                };

                Ok(Self::UnaryOp {
                    op: op.into(),
                    rhs: Box::new(rhs?),
                    src_ref: pair.clone().into(),
                })
            })
            .map_postfix(|lhs, op| {
                match (Pair::new(op.clone(), pair.source_hash()), op.as_rule()) {
                    (op, Rule::list_element_access) => Ok(Self::ListElementAccess(
                        Box::new(lhs?),
                        Box::new(Self::parse(op)?),
                        pair.clone().into(),
                    )),
                    (op, Rule::tuple_element_access) => {
                        let op = op.inner().next().expect(INTERNAL_PARSE_ERROR);
                        match op.as_rule() {
                            Rule::identifier => Ok(Self::NamedTupleElementAccess(
                                Box::new(lhs?),
                                Identifier::parse(op)?,
                                pair.clone().into(),
                            )),
                            Rule::int => Ok(Self::UnnamedTupleElementAccess(
                                Box::new(lhs?),
                                op.as_str().parse().expect("Integer expression expected"),
                                pair.clone().into(),
                            )),
                            rule => unreachable!("Expected identifier or int, found {:?}", rule),
                        }
                    }
                    (op, Rule::method_call) => Ok(Self::MethodCall(
                        Box::new(lhs?),
                        MethodCall::parse(op)?,
                        pair.clone().into(),
                    )),
                    rule => {
                        unreachable!("Expr::parse expected postfix operation, found {:?}", rule)
                    }
                }
            })
            .parse(pair.pest_pair().clone().into_inner())
    }
}
