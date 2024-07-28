use crate::eval;
use crate::expression::Expression;
use crate::langtype::Type;
use crate::parser::{Pair, Parse, ParseError, Rule};
use crate::units::Unit;

/// Definition and implementation for `NumberLiteral`
#[derive(Debug, Clone)]
pub struct NumberLiteral(pub f64, pub Unit);

impl NumberLiteral {
    pub fn from_usize(value: usize) -> Self {
        NumberLiteral(value as f64, Unit::None)
    }

    pub fn ty(&self) -> Type {
        self.1.ty()
    }

    /// Returns the actual value of the literal
    pub fn value(&self) -> f64 {
        self.1.normalize(self.0)
    }
}

/// Rules for operator +
impl std::ops::Add for &NumberLiteral {
    type Output = Option<NumberLiteral>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.ty(), rhs.ty()) {
            (Type::Scalar, Type::Scalar) => {
                Some(NumberLiteral(self.value() + rhs.value(), Unit::None))
            }
            (Type::Angle, Type::Angle) => {
                Some(NumberLiteral(self.value() + rhs.value(), Unit::Deg))
            }
            (Type::Length, Type::Length) => {
                Some(NumberLiteral(self.value() + rhs.value(), Unit::Mm))
            }
            _ => None,
        }
    }
}

/// Rules for operator -
impl std::ops::Sub for &NumberLiteral {
    type Output = Option<NumberLiteral>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.ty(), rhs.ty()) {
            (Type::Scalar, Type::Scalar) => {
                Some(NumberLiteral(self.value() - rhs.value(), Unit::None))
            }
            (Type::Angle, Type::Angle) => {
                Some(NumberLiteral(self.value() - rhs.value(), Unit::Deg))
            }
            (Type::Length, Type::Length) => {
                Some(NumberLiteral(self.value() - rhs.value(), Unit::Mm))
            }
            _ => None,
        }
    }
}

/// Rules for operator *
impl std::ops::Mul for &NumberLiteral {
    type Output = Option<NumberLiteral>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self.ty(), rhs.ty()) {
            (Type::Scalar, Type::Scalar) => {
                Some(NumberLiteral(self.value() * rhs.value(), Unit::None))
            }
            (Type::Angle, Type::Scalar) | (Type::Scalar, Type::Angle) => {
                Some(NumberLiteral(self.value() * rhs.value(), Unit::Deg))
            }
            (Type::Length, Type::Scalar) | (Type::Scalar, Type::Length) => {
                Some(NumberLiteral(self.value() * rhs.value(), Unit::Mm))
            }
            _ => None,
        }
    }
}

/// Rules for operator -
impl std::ops::Div for &NumberLiteral {
    type Output = Option<NumberLiteral>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self.ty(), rhs.ty()) {
            (Type::Scalar, Type::Scalar)
            | (Type::Length, Type::Length)
            | (Type::Angle, Type::Angle) => {
                Some(NumberLiteral(self.value() / rhs.value(), Unit::None))
            }
            (Type::Angle, Type::Scalar) => {
                Some(NumberLiteral(self.value() / rhs.value(), Unit::Deg))
            }
            (Type::Length, Type::Scalar) => {
                Some(NumberLiteral(self.value() / rhs.value(), Unit::Mm))
            }
            _ => None,
        }
    }
}

impl ToString for NumberLiteral {
    fn to_string(&self) -> String {
        format!("{}{}", self.0.to_string(), self.1)
    }
}

impl Parse for NumberLiteral {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        assert_eq!(pair.as_rule(), Rule::number_literal);

        let mut pairs = pair.into_inner();
        let number_token = pairs.next().unwrap();

        assert_eq!(number_token.as_rule(), Rule::number);

        let value = number_token.as_str().parse::<f64>()?;
        let mut unit = Unit::None;

        if let Some(unit_token) = pairs.next() {
            unit = Unit::parse(unit_token)?;
        }
        Ok(NumberLiteral(value, unit))
    }
}

impl eval::Eval for NumberLiteral {
    fn eval(self, _: Option<&eval::Context>) -> Result<Box<Expression>, eval::Error> {
        Ok(Box::new(Expression::NumberLiteral(NumberLiteral(
            self.value(),
            self.1.ty().default_unit(),
        ))))
    }

    fn eval_type(&self, _: Option<&eval::Context>) -> Result<Type, eval::Error> {
        Ok(self.1.ty())
    }
}
