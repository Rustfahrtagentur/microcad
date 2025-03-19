// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad literal parser entities

mod color;
mod number_literal;
mod units;

pub use color::*;
pub use number_literal::*;
pub use units::*;

use crate::{parse::*, parser::*, src_ref::*, r#type::*};

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

impl crate::ty::Ty for Literal {
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

impl Syntax for Literal {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        write!(f, "{:depth$}Literal: ", "")?;
        match self {
            Literal::Integer(i) => writeln!(f, "{}", i),
            Literal::Number(n) => writeln!(f, "{}", n),
            Literal::Bool(b) => writeln!(f, "{}", b),
            Literal::Color(c) => writeln!(f, "{}", c),
        }
    }
}
