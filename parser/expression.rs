use crate::{units::Unit, CsglParser};
use pest::pratt_parser::PrattParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<crate::Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use crate::Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
    };
}

#[derive(Default)]
pub enum Expression {
    /// Something went wrong (and an error will be reported)
    #[default]
    Invalid,

    /// A string literal. The .0 is the content of the string, without the quotes
    StringLiteral(String),
    /// Number
    NumberLiteral(f64, Unit),
    /// Bool
    BoolLiteral(bool),

    BinaryOp {
        lhs: Box<Expression>,
        /// '+', '-', '/', '*', '=', '!', '<', '>', '≤', '≥', '&', '|'
        op: char,
        rhs: Box<Expression>,
    },

    UnaryOp {
        /// '+', '-', '!'
        op: char,
        rhs: Box<Expression>,
    },
}

impl crate::Parse for Expression {
    fn parse(pair: pest::iterators::Pair<crate::Rule>) -> Result<Self, crate::ParseError> {
        use crate::Rule;

        Ok(PRATT_PARSER
            .map_primary(|primary| match primary.as_rule() {
                Rule::number_literal => {
                    let (number, unit) = CsglParser::number_literal(primary).unwrap();
                    Expression::NumberLiteral(number, unit)
                }
                Rule::expression => Self::parse(primary).unwrap(),
                rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
            })
            .map_infix(|lhs, op, rhs| {
                let op = match op.as_rule() {
                    Rule::add => '+',
                    Rule::subtract => '-',
                    Rule::multiply => '*',
                    Rule::divide => '/',
                    rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
                };
                Expression::BinaryOp {
                    lhs: Box::new(lhs),
                    op,
                    rhs: Box::new(rhs),
                }
            })
            .map_prefix(|op, rhs| {
                let op = match op.as_rule() {
                    Rule::unary_minus => '-',
                    Rule::unary_plus => '+',
                    Rule::unary_not => '!',
                    _ => unreachable!(),
                };

                Expression::UnaryOp {
                    op,
                    rhs: Box::new(rhs),
                }
            })
            .parse(pair.into_inner()))
    }
}
