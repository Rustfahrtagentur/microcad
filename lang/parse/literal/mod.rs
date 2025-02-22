// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad literal parser entities

mod color;
mod number_literal;
mod units;

pub use color::*;
pub use number_literal::*;
pub use units::*;

use crate::{eval::*, parse::*, parser::*, r#type::*, src_ref::*};

/// Literal entity
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    /// Integer literal
    Integer(Refer<i64>),
    /// Number literal
    Number(NumberLiteral),
    /// Boolean literal
    Bool(Refer<bool>),
    /// Color literal
    Color(Refer<Color>),
}

impl SrcReferrer for Literal {
    fn src_ref(&self) -> SrcRef {
        match self {
            Literal::Number(n) => n.src_ref(),
            Literal::Integer(i) => i.src_ref(),
            Literal::Bool(b) => b.src_ref(),
            Literal::Color(c) => c.src_ref(),
        }
    }
}

impl std::str::FromStr for Literal {
    type Err = ParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Parser::parse_rule::<Self>(Rule::literal, s, 0)
    }
}

impl Ty for Literal {
    fn ty(&self) -> Type {
        match self {
            Literal::Integer(_) => Type::Integer,
            Literal::Number(n) => n.ty(),
            Literal::Bool(_) => Type::Bool,
            Literal::Color(_) => Type::Color,
        }
    }
}

impl Parse for Literal {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::literal);

        let inner = pair.inner().next().expect(INTERNAL_PARSE_ERROR);

        let s = match inner.as_rule() {
            Rule::number_literal => Literal::Number(NumberLiteral::parse(inner)?),
            Rule::integer_literal => {
                Literal::Integer(Refer::new(inner.as_str().parse::<i64>()?, pair.into()))
            }
            Rule::bool_literal => match inner.as_str() {
                "true" => Literal::Bool(Refer::new(true, pair.into())),
                "false" => Literal::Bool(Refer::new(false, pair.into())),
                _ => unreachable!(),
            },
            Rule::color_literal => Literal::Color(Refer::new(Color::parse(inner)?, pair.into())),
            _ => unreachable!(),
        };

        Ok(s)
    }
}

impl Eval for Literal {
    type Output = Value;

    fn eval(&self, context: &mut EvalContext) -> std::result::Result<Value, EvalError> {
        match self {
            Literal::Integer(i) => Ok(Value::Integer(i.clone().map(|i| i))),
            Literal::Number(n) => n.eval(context),
            Literal::Bool(b) => Ok(Value::Bool(b.clone().map(|b| b))),
            Literal::Color(c) => Ok(Value::Color(c.clone().map(|c| c))),
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Literal::Integer(i) => write!(f, "{}", i),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Color(c) => write!(f, "{}", c),
        }
    }
}
