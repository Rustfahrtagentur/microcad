mod number_literal;

pub use number_literal::NumberLiteral;

use crate::{eval::*, language::*, parser::*};

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Number(NumberLiteral),
    Bool(bool),
    Color(Color),
}

impl Literal {
    pub fn number_unit(n: f64, u: Unit) -> Self {
        Self::Number(NumberLiteral(n, u))
    }
}

impl std::str::FromStr for Literal {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Parser::parse_rule::<Self>(Rule::literal, s)
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
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        Parser::ensure_rule(&pair, Rule::literal);
        use crate::with_pair_ok;

        let inner = pair.clone().into_inner().next().unwrap();

        let s = match inner.as_rule() {
            Rule::number_literal => Literal::Number(NumberLiteral::parse(inner)?.value().clone()),
            Rule::integer_literal => Literal::Integer(inner.as_str().parse::<i64>()?),
            Rule::bool_literal => match inner.as_str() {
                "true" => Literal::Bool(true),
                "false" => Literal::Bool(false),
                _ => unreachable!(),
            },
            Rule::color_literal => Literal::Color(*Color::parse(inner)?),
            _ => unreachable!(),
        };

        with_pair_ok!(s, pair)
    }
}

impl Eval for Literal {
    type Output = Value;

    fn eval(&self, context: &mut Context) -> Result<Value, EvalError> {
        match self {
            Literal::Integer(i) => Ok(Value::Integer(*i)),
            Literal::Number(n) => n.eval(context),
            Literal::Bool(b) => Ok(Value::Bool(*b)),
            Literal::Color(c) => Ok(Value::Color(*c)),
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

impl std::ops::Add for Literal {
    type Output = Result<Self, OperatorError>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Literal::Number(n1), Literal::Number(n2)) => Ok(Literal::Number((n1 + n2)?)),
            (Literal::Integer(i1), Literal::Integer(i2)) => Ok(Literal::Integer(i1 + i2)),
            (Literal::Number(n), Literal::Integer(i))
            | (Literal::Integer(i), Literal::Number(n)) => {
                Ok(Literal::Number((n + NumberLiteral::from_int(i))?))
            }
            (l, r) => Err(OperatorError::AddIncompatibleTypes(l.ty(), r.ty())),
        }
    }
}

impl std::ops::Sub for Literal {
    type Output = Result<Self, OperatorError>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Literal::Number(n1), Literal::Number(n2)) => Ok(Literal::Number((n1 - n2)?)),
            (Literal::Integer(i1), Literal::Integer(i2)) => Ok(Literal::Integer(i1 - i2)),
            (Literal::Number(n), Literal::Integer(i)) => {
                Ok(Literal::Number((n - NumberLiteral::from_int(i))?))
            }
            (Literal::Integer(i), Literal::Number(n)) => {
                Ok(Literal::Number((NumberLiteral::from_int(i) - n)?))
            }
            (l, r) => Err(OperatorError::SubIncompatibleTypes(l.ty(), r.ty())),
        }
    }
}

impl std::ops::Mul for Literal {
    type Output = Result<Self, OperatorError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Literal::Number(n1), Literal::Number(n2)) => Ok(Literal::Number((n1 * n2)?)),
            (Literal::Integer(i1), Literal::Integer(i2)) => Ok(Literal::Integer(i1 * i2)),
            (Literal::Number(n), Literal::Integer(i))
            | (Literal::Integer(i), Literal::Number(n)) => {
                Ok(Literal::Number((n * NumberLiteral::from_int(i))?))
            }
            (l, r) => Err(OperatorError::MulIncompatibleTypes(l.ty(), r.ty())),
        }
    }
}

impl std::ops::Div for Literal {
    type Output = Result<Self, OperatorError>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Literal::Number(n1), Literal::Number(n2)) => Ok(Literal::Number((n1 / n2)?)),
            (Literal::Integer(i1), Literal::Integer(i2)) => Ok(Literal::Number(
                (NumberLiteral::from_int(i1) / NumberLiteral::from_int(i2))?,
            )),
            (Literal::Number(n), Literal::Integer(i)) => {
                Ok(Literal::Number((n / NumberLiteral::from_int(i))?))
            }
            (Literal::Integer(i), Literal::Number(n)) => {
                Ok(Literal::Number((NumberLiteral::from_int(i) / n)?))
            }
            (l, r) => Err(OperatorError::DivIncompatibleTypes(l.ty(), r.ty())),
        }
    }
}
