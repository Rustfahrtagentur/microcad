use std::os::windows::io::InvalidHandleError;

use crate::{literal::NumberLiteral, units::Unit, CsglParser};
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
    NumberLiteral(NumberLiteral),
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

pub enum EvalError {
    InvalidOperation,
}

impl std::ops::Mul for Box<Expression> {
    type Output = Result<Box<Expression>, EvalError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self.as_ref(), rhs.as_ref()) {
            (Expression::NumberLiteral(lhs), Expression::NumberLiteral(rhs)) => match lhs * rhs {
                Some(result) => Ok(Box::new(Expression::NumberLiteral(result))),
                None => Err(EvalError::InvalidOperation),
            },
            _ => Err(EvalError::InvalidOperation),
        }
    }
}

impl Expression {
    fn eval(self) -> Result<Box<Self>, EvalError> {
        match self {
            Self::NumberLiteral(_) | Self::StringLiteral(_) => Ok(Box::new(self)),
            Self::BinaryOp { lhs, op, rhs } => match op {
                '*' => lhs * rhs,
                _ => unimplemented!(),
            },
            _ => Err(EvalError::InvalidOperation),
        }
    }
}

impl crate::Parse for Expression {
    fn parse(pair: pest::iterators::Pair<crate::Rule>) -> Result<Self, crate::ParseError> {
        use crate::Rule;

        Ok(PRATT_PARSER
            .map_primary(|primary| match primary.as_rule() {
                Rule::literal => {
                    let inner = primary.into_inner().next().unwrap();

                    match inner.as_rule() {
                        Rule::number_literal => {
                            let number_literal = CsglParser::number_literal(inner).unwrap();
                            Expression::NumberLiteral(number_literal)
                        }
                        rule => unreachable!("Expr::parse expected literal, found {:?}", rule),
                    }
                }
                Rule::number_literal => {
                    let number_literal = CsglParser::number_literal(primary).unwrap();
                    Expression::NumberLiteral(number_literal)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expression() {
        use pest::Parser;
        let pair = CsglParser::parse(crate::Rule::expression, "4 * 4")
            .unwrap()
            .next()
            .unwrap();

        use crate::Parse;
        let expr = Expression::parse(pair).unwrap();

        match expr {
            Expression::BinaryOp { lhs: _, op, rhs: _ } => {
                assert_eq!(op, '*');
            }
            _ => panic!("Wrong Expression Type"),
        }
    }
}
