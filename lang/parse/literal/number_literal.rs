use crate::{eval::*, parse::*, parser::*, r#type::*};

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
    type Output = std::result::Result<Self, OperatorError>;

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
    type Output = std::result::Result<Self, OperatorError>;

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
    type Output = std::result::Result<Self, OperatorError>;

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
    type Output = std::result::Result<Self, OperatorError>;

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

    fn eval(&self, _: &mut Context) -> std::result::Result<Value, EvalError> {
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
