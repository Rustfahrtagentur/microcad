mod number_literal;

pub use number_literal::NumberLiteral;

use crate::{errors::*, eval::*, parse::*, parser::*, r#type::*, src_ref::*};

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64, SrcRef),
    Number(NumberLiteral),
    Bool(bool, SrcRef),
    Color(Color, SrcRef),
}

impl SrcReferrer for Literal {
    fn src_ref(&self) -> SrcRef {
        match self {
            Literal::Number(n) => n.src_ref(),
            Literal::Integer(_, r) | Literal::Bool(_, r) | Literal::Color(_, r) => r.clone(),
        }
    }
}

impl std::str::FromStr for Literal {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Parser::parse_rule::<Self>(Rule::literal, s)
    }
}

impl Ty for Literal {
    fn ty(&self) -> Type {
        match self {
            Literal::Integer(_, _) => Type::Integer,
            Literal::Number(n) => n.ty(),
            Literal::Bool(_, _) => Type::Bool,
            Literal::Color(_, _) => Type::Color,
        }
    }
}

impl Parse for Literal {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::literal);

        let inner = pair.clone().into_inner().next().unwrap();

        let s = match inner.as_rule() {
            Rule::number_literal => Literal::Number(NumberLiteral::parse(inner)?),
            Rule::integer_literal => Literal::Integer(inner.as_str().parse::<i64>()?, pair.into()),
            Rule::bool_literal => match inner.as_str() {
                "true" => Literal::Bool(true, pair.into()),
                "false" => Literal::Bool(false, pair.into()),
                _ => unreachable!(),
            },
            Rule::color_literal => Literal::Color(Color::parse(inner)?, pair.into()),
            _ => unreachable!(),
        };

        Ok(s)
    }
}

impl Eval for Literal {
    type Output = Value;

    fn eval(&self, context: &mut Context) -> std::result::Result<Value, EvalError> {
        match self {
            Literal::Integer(i, _) => Ok(Value::Integer(*i)),
            Literal::Number(n) => n.eval(context),
            Literal::Bool(b, _) => Ok(Value::Bool(*b)),
            Literal::Color(c, _) => Ok(Value::Color(*c)),
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Literal::Integer(i, _) => write!(f, "{}", i),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Bool(b, _) => write!(f, "{}", b),
            Literal::Color(c, _) => write!(f, "{}", c),
        }
    }
}
