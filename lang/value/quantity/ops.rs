// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Quantity binery operators module.

use microcad_core::{Integer, Scalar};

use crate::{ty::*, value::*};

impl std::ops::Neg for Quantity {
    type Output = Quantity;

    fn neg(self) -> Self::Output {
        Self {
            value: -self.value,
            quantity_type: self.quantity_type,
        }
    }
}

impl std::ops::Add for Quantity {
    type Output = QuantityResult;

    fn add(self, rhs: Self) -> Self::Output {
        if self.quantity_type == rhs.quantity_type {
            Ok(Quantity::new(self.value + rhs.value, self.quantity_type))
        } else {
            Err(QuantityError::InvalidOperation(self, '+', rhs))
        }
    }
}

impl std::ops::Add<Integer> for Quantity {
    type Output = QuantityResult;

    fn add(self, rhs: Integer) -> Self::Output {
        if self.quantity_type == QuantityType::Scalar {
            Ok(Quantity::new(
                self.value + rhs as Scalar,
                self.quantity_type,
            ))
        } else {
            Err(QuantityError::InvalidOperation(self, '+', rhs.into()))
        }
    }
}

impl std::ops::Add<Quantity> for Integer {
    type Output = QuantityResult;

    fn add(self, rhs: Quantity) -> Self::Output {
        if rhs.quantity_type == QuantityType::Scalar {
            Ok(Quantity::new(self as Scalar + rhs.value, rhs.quantity_type))
        } else {
            Err(QuantityError::InvalidOperation(self.into(), '+', rhs))
        }
    }
}

impl std::ops::Sub for Quantity {
    type Output = QuantityResult;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.quantity_type == rhs.quantity_type {
            Ok(Quantity::new(self.value - rhs.value, self.quantity_type))
        } else {
            Err(QuantityError::InvalidOperation(self, '-', rhs))
        }
    }
}

impl std::ops::Sub<Integer> for Quantity {
    type Output = QuantityResult;

    fn sub(self, rhs: Integer) -> Self::Output {
        if self.quantity_type == QuantityType::Scalar {
            Ok(Quantity::new(
                self.value - rhs as Scalar,
                self.quantity_type,
            ))
        } else {
            Err(QuantityError::InvalidOperation(self, '-', rhs.into()))
        }
    }
}

impl std::ops::Sub<Quantity> for Integer {
    type Output = QuantityResult;

    fn sub(self, rhs: Quantity) -> Self::Output {
        if rhs.quantity_type == QuantityType::Scalar {
            Ok(Quantity::new(self as Scalar - rhs.value, rhs.quantity_type))
        } else {
            Err(QuantityError::InvalidOperation(self.into(), '-', rhs))
        }
    }
}

impl std::ops::Mul for Quantity {
    type Output = QuantityResult;

    fn mul(self, rhs: Self) -> Self::Output {
        let t = self.quantity_type.clone() * rhs.quantity_type.clone();
        if t == QuantityType::Invalid {
            Err(QuantityError::InvalidOperation(self, '*', rhs))
        } else {
            Ok(Self::new(self.value * rhs.value, t))
        }
    }
}

impl std::ops::Mul<Integer> for Quantity {
    type Output = QuantityResult;

    fn mul(self, rhs: Integer) -> Self::Output {
        self * Into::<Quantity>::into(rhs)
    }
}

impl std::ops::Mul<Quantity> for Integer {
    type Output = QuantityResult;

    fn mul(self, rhs: Quantity) -> Self::Output {
        Into::<Quantity>::into(self) * rhs
    }
}

impl std::ops::Div for Quantity {
    type Output = QuantityResult;

    fn div(self, rhs: Self) -> Self::Output {
        let t = self.quantity_type.clone() / rhs.quantity_type.clone();
        if t == QuantityType::Invalid {
            Err(QuantityError::InvalidOperation(self, '/', rhs))
        } else {
            Ok(Self::new(self.value / rhs.value, t))
        }
    }
}

impl std::ops::Div<Integer> for Quantity {
    type Output = QuantityResult;

    fn div(self, rhs: Integer) -> Self::Output {
        self / Into::<Quantity>::into(rhs)
    }
}

impl std::ops::Div<Quantity> for Integer {
    type Output = QuantityResult;

    fn div(self, rhs: Quantity) -> Self::Output {
        Into::<Quantity>::into(self) / rhs
    }
}
