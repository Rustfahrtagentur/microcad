use crate::{parse::*, parser::*, syntax::*};

impl Parse for ExpressionList {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(Self(
            pair.inner()
                .map(Expression::parse)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl Parse for ListExpression {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut inner = pair.inner();
        Ok(Self {
            list: ExpressionList::parse(inner.next().expect("list_expression expected"))?,
            unit: match inner.next() {
                Some(pair) => Some(Unit::parse(pair)?),
                None => None,
            },
            src_ref: pair.clone().into(),
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
        let call_argument_list =
            CallArgumentList::parse(inner.next().expect(INTERNAL_PARSE_ERROR))?;
        if call_argument_list.is_empty() {
            return Err(ParseError::EmptyTupleExpression);
        }

        // Count number of positional and named arguments
        let named_count: usize = call_argument_list
            .iter()
            .map(|c| if c.name.is_some() { 1 } else { 0 })
            .sum();

        if named_count > 0 && named_count < call_argument_list.len() {
            return Err(ParseError::MixedTupleArguments);
        }

        Ok(TupleExpression {
            is_named: named_count == call_argument_list.len(),
            args: call_argument_list,
            unit: match inner.next() {
                Some(pair) => Some(Unit::parse(pair)?),
                None => None,
            },
            src_ref: pair.clone().into(),
        })
    }
}
