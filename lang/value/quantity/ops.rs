// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::Integer;

use crate::value::{Quantity, error::QuantityResult};

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
    type Output = Quantity;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Add<Integer> for Quantity {
    type Output = QuantityResult;

    fn add(self, rhs: Integer) -> Self::Output {
        todo!()
    }
}

impl std::ops::Add<Quantity> for Integer {
    type Output = QuantityResult;

    fn add(self, rhs: Quantity) -> Self::Output {
        todo!()
    }
}

impl std::ops::Sub for Quantity {
    type Output = Quantity;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Sub<Integer> for Quantity {
    type Output = QuantityResult;

    fn sub(self, rhs: Integer) -> Self::Output {
        todo!()
    }
}

impl std::ops::Sub<Quantity> for Integer {
    type Output = QuantityResult;

    fn sub(self, rhs: Quantity) -> Self::Output {
        todo!()
    }
}

impl std::ops::Mul for Quantity {
    type Output = Quantity;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Mul<Integer> for Quantity {
    type Output = Quantity;

    fn mul(self, rhs: Integer) -> Self::Output {
        todo!()
    }
}

impl std::ops::Mul<Quantity> for Integer {
    type Output = Quantity;

    fn mul(self, rhs: Quantity) -> Self::Output {
        todo!()
    }
}

impl std::ops::Div for Quantity {
    type Output = QuantityResult;

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Div<Integer> for Quantity {
    type Output = QuantityResult;

    fn div(self, rhs: Integer) -> Self::Output {
        todo!()
    }
}

impl std::ops::Div<Quantity> for Integer {
    type Output = QuantityResult;

    fn div(self, rhs: Quantity) -> Self::Output {
        todo!()
    }
}
