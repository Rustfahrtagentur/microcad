// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::*};

impl Parse for ExpressionList {
    fn parse(pair: Pair) -> ParseResult<Self> {
        pair.inner()
            .filter_map(|pair| match pair.as_rule() {
                Rule::expression | Rule::expression_no_semicolon => Some(Expression::parse(pair)),
                _ => None,
            })
            .collect::<Result<Vec<_>, _>>()
    }
}

impl Parse for ListExpression {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(Self {
            list: pair
                .find(Rule::expression_list)
                .expect("expression_list expected"),
            unit: pair.find(Rule::unit).unwrap_or_default(),
            src_ref: pair.clone().into(),
        })
    }
}

impl Parse for Marker {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::marker);
        Ok(Self {
            id: Identifier::parse(pair.inner().next().expect(INTERNAL_PARSE_ERROR))?,
            src_ref: pair.src_ref(),
        })
    }
}

lazy_static::lazy_static! {
    /// Expression parser
    static ref PRATT_PARSER: pest::pratt_parser::PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc, Op,PrattParser};
        use Assoc::*;
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(or, Left) | Op::infix(and, Left))
            .op(Op::infix(equal, Left) | Op::infix(not_equal, Left))
            .op(Op::infix(greater_than, Left) | Op::infix(less_than, Left))
            .op(Op::infix(less_equal, Left) | Op::infix(greater_equal, Left))
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
            .op(Op::infix(r#union, Left) | Op::infix(intersection, Left))
            .op(Op::infix(power_xor, Left))
            .op(Op::infix(near, Left))
            .op(Op::prefix(unary_minus))
            .op(Op::prefix(unary_plus))
            .op(Op::prefix(unary_not))
            .op(Op::postfix(method_call))
            .op(Op::postfix(list_element_access))
            .op(Op::postfix(tuple_element_access))
            .op(Op::postfix(attribute_access))
    };
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
}

impl Parse for Expression {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rules(&pair, &[Rule::expression_no_semicolon, Rule::expression]);

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
                    (primary, Rule::marker) => Ok(Self::Marker(Marker::parse(primary)?)),
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
                    Rule::equal => "==",
                    Rule::near => "~",
                    Rule::not_equal => "!=",
                    Rule::and => "&",
                    Rule::or => "|",

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
                    (op, Rule::list_element_access) => Ok(Self::ArrayElementAccess(
                        Box::new(lhs?),
                        Box::new(Self::parse(op)?),
                        pair.clone().into(),
                    )),
                    (op, Rule::attribute_access) => {
                        let op = op.inner().next().expect(INTERNAL_PARSE_ERROR);
                        Ok(Self::AttributeAccess(
                            Box::new(lhs?),
                            Identifier::parse(op)?,
                            pair.clone().into(),
                        ))
                    }
                    (op, Rule::tuple_element_access) => {
                        let op = op.inner().next().expect(INTERNAL_PARSE_ERROR);
                        match op.as_rule() {
                            Rule::identifier => Ok(Self::PropertyAccess(
                                Box::new(lhs?),
                                Identifier::parse(op)?,
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
            .parse(
                pair.pest_pair()
                    .clone()
                    .into_inner()
                    .filter(|pair| pair.as_rule() != Rule::COMMENT), // Filter comments
            )
    }
}

impl Parse for Nested {
    fn parse(pair: Pair) -> ParseResult<Self> {
        assert!(pair.as_rule() == Rule::nested || pair.as_rule() == Rule::expression_no_semicolon);

        Ok(Self(Refer::new(
            pair.inner()
                .filter(|pair| {
                    [Rule::qualified_name, Rule::call, Rule::body].contains(&pair.as_rule())
                })
                .map(NestedItem::parse)
                .collect::<ParseResult<_>>()?,
            pair.src_ref(),
        )))
    }
}

impl Parse for NestedItem {
    fn parse(pair: Pair) -> ParseResult<Self> {
        match pair.clone().as_rule() {
            Rule::call => Ok(Self::Call(Call::parse(pair.clone())?)),
            Rule::qualified_name => Ok(Self::QualifiedName(QualifiedName::parse(pair.clone())?)),
            Rule::body => Ok(Self::Body(Body::parse(pair.clone())?)),
            rule => unreachable!(
                "NestedItem::parse expected call or qualified name, found {:?}",
                rule
            ),
        }
    }
}

impl Parse for TupleExpression {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut inner = pair.inner();
        let argument_list = ArgumentList::parse(inner.next().expect(INTERNAL_PARSE_ERROR))?;
        if argument_list.is_empty() {
            return Err(ParseError::EmptyTupleExpression);
        }

        Ok(TupleExpression {
            args: argument_list,
            unit: match inner.next() {
                Some(pair) => Unit::parse(pair)?,
                None => Unit::None,
            },
            src_ref: pair.clone().into(),
        })
    }
}

/// Create TupleExpression from µcad code
#[macro_export]
macro_rules! tuple_expression {
    ($code:literal) => {{
        $crate::parse!(
            TupleExpression,
            $crate::parser::Rule::tuple_expression,
            $code
        )
    }};
}
