use crate::units::Unit;
use pest::pratt_parser::PrattParser;

type Op = pest::pratt_parser::Op<crate::Rule>;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<crate::Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use crate::Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(add, Left) | Op::infix(sub, Left))
            .op(Op::infix(mul, Left) | Op::infix(div, Left))
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

    BinOp {
        lhs: Box<Expression>,
        op: Op,
        rhs: Box<Expression>,
    },

    UnaryOp {
        sub: Box<Expression>,
        /// '+', '-', '!'
        op: char,
    },
}
