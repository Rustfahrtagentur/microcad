use super::{color::*, lang_type::*, units::*, value::*};
use crate::{eval::*, parser::*};

/// Definition and implementation for `NumberLiteral`
#[derive(Debug, Clone, PartialEq)]
pub struct NumberLiteral(pub f64, pub Unit);

impl NumberLiteral {
    pub fn from_usize(value: usize) -> Self {
        NumberLiteral(value as f64, Unit::None)
    }

    pub fn from_int(value: i64) -> Self {
        NumberLiteral(value as f64, Unit::None)
    }

    pub fn ty(&self) -> Type {
        if self.1 == Unit::None && self.0.fract() == 0.0 {
            return Type::Integer;
        }
        self.1.ty()
    }

    /// Returns the actual value of the literal
    pub fn value(&self) -> f64 {
        self.1.normalize(self.0)
    }

    pub fn unit(&self) -> Unit {
        self.1
    }
}

/// Rules for operator +
impl std::ops::Add for NumberLiteral {
    type Output = Result<Self, OperatorError>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.ty(), rhs.ty()) {
            (Type::Scalar, Type::Scalar) => {
                Ok(NumberLiteral(self.value() + rhs.value(), Unit::None))
            }
            (Type::Angle, Type::Angle) => Ok(NumberLiteral(self.value() + rhs.value(), Unit::Deg)),
            (Type::Length, Type::Length) => Ok(NumberLiteral(self.value() + rhs.value(), Unit::Mm)),
            (l, r) => Err(OperatorError::AddIncompatibleTypes(l, r)),
        }
    }
}

/// Rules for operator -
impl std::ops::Sub for NumberLiteral {
    type Output = Result<Self, OperatorError>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.ty(), rhs.ty()) {
            (Type::Scalar, Type::Scalar) => {
                Ok(NumberLiteral(self.value() - rhs.value(), Unit::None))
            }
            (Type::Angle, Type::Angle) => Ok(NumberLiteral(self.value() - rhs.value(), Unit::Deg)),
            (Type::Length, Type::Length) => Ok(NumberLiteral(self.value() - rhs.value(), Unit::Mm)),
            (l, r) => Err(OperatorError::SubIncompatibleTypes(l, r)),
        }
    }
}

/// Rules for operator *
impl std::ops::Mul for NumberLiteral {
    type Output = Result<Self, OperatorError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self.ty(), rhs.ty()) {
            (Type::Scalar, Type::Scalar) => {
                Ok(NumberLiteral(self.value() * rhs.value(), Unit::None))
            }
            (Type::Angle, Type::Scalar) => {
                Ok(NumberLiteral(self.value() * rhs.value(), self.unit()))
            }
            (Type::Scalar, Type::Angle) => {
                Ok(NumberLiteral(self.value() * rhs.value(), rhs.unit()))
            }
            (Type::Length, Type::Scalar) => {
                Ok(NumberLiteral(self.value() * rhs.value(), self.unit()))
            }
            (Type::Scalar, Type::Length) => {
                Ok(NumberLiteral(self.value() * rhs.value(), rhs.unit()))
            }
            (l, r) => Err(OperatorError::MulIncompatibleTypes(l, r)),
        }
    }
}

/// Rules for operator -
impl std::ops::Div for NumberLiteral {
    type Output = Result<Self, OperatorError>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self.ty(), rhs.ty()) {
            (Type::Scalar, Type::Scalar)
            | (Type::Length, Type::Length)
            | (Type::Angle, Type::Angle) => {
                Ok(NumberLiteral(self.value() / rhs.value(), Unit::None))
            }
            (Type::Angle, Type::Scalar) => Ok(NumberLiteral(self.value() / rhs.value(), Unit::Deg)),
            (Type::Length, Type::Scalar) => Ok(NumberLiteral(self.value() / rhs.value(), Unit::Mm)),
            (l, r) => Err(OperatorError::DivIncompatibleTypes(l, r)),
        }
    }
}

impl Parse for NumberLiteral {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        Parser::ensure_rule(&pair, Rule::number_literal);

        let mut inner = pair.clone().into_inner();
        let number_token = inner.next().unwrap();

        assert!(
            number_token.as_rule() == Rule::number
                || number_token.as_rule() == Rule::integer_literal
        );

        let value = number_token.as_str().parse::<f64>()?;

        let mut unit = Unit::None;

        if let Some(unit_token) = inner.next() {
            unit = *Unit::parse(unit_token)?;
        }
        Ok(WithPair::new(NumberLiteral(value, unit), pair))
    }
}

impl Eval for NumberLiteral {
    type Output = Value;

    fn eval(&self, _: &mut Context) -> Result<Value, Error> {
        let v = self.value();

        match self.1.ty() {
            Type::Scalar => Ok(Value::Scalar(v)),
            Type::Angle => Ok(Value::Angle(v)),
            Type::Length => Ok(Value::Length(v)),
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for NumberLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

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

    fn eval(&self, context: &mut Context) -> Result<Value, Error> {
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