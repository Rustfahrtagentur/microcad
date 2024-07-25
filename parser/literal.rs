use crate::langtype::Type;
use crate::units::Unit;

pub struct NumberLiteral(pub f64, pub Unit);

impl NumberLiteral {
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
