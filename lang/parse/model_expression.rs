// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::*};

impl Parse for ModelExpressionList {
    fn parse(pair: Pair) -> ParseResult<Self> {
        pair.inner()
            .filter_map(|pair| match pair.as_rule() {
                Rule::model_expression => Some(ModelExpression::parse(pair)),
                _ => None,
            })
            .collect::<Result<Vec<_>, _>>()
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
            .op(Op::infix(difference, Left) | Op::infix(r#union, Left) | Op::infix(intersection, Left))
            .op(Op::postfix(method_call))
            .op(Op::postfix(attribute_access))
    };
}

impl Parse for ModelExpression {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rules(&pair, &[Rule::model_expression]);

        PRATT_PARSER
            .map_primary(|primary| {
                match (
                    Pair::new(primary.clone(), pair.source_hash()),
                    primary.as_rule(),
                ) {
                    (primary, Rule::model_expression) => Ok(Self::parse(primary)?),
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
                    Rule::difference => "-",
                    Rule::r#union => "|",
                    Rule::intersection => "&",
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
