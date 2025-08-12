// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Matrix value type

pub mod error;
pub mod ops;

use crate::ty::*;
use microcad_core::*;

pub use error::*;

/// A numeric value
#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Quantity {
    /// The numeric value of the quantity.
    pub value: Scalar,
    /// The quantity type with a base unit.
    pub quantity_type: QuantityType,
}

impl Quantity {
    /// Create a new quantity.
    pub fn new(value: Scalar, quantity_type: QuantityType) -> Self {
        Self {
            value,
            quantity_type,
        }
    }

    /// Create a new Scalar quantity.
    pub fn scalar(value: Scalar) -> Self {
        Quantity::new(value, QuantityType::Scalar)
    }

    /// Create a new Length quantity in millimeters.
    pub fn length(length: Scalar) -> Self {
        Quantity::new(length, QuantityType::Length)
    }

    /// Calculate the power of quantity.
    ///
    /// *Note: This function has not been implemented completely.*
    pub fn pow(&self, rhs: &Quantity) -> Self {
        match (&self.quantity_type, &rhs.quantity_type) {
            (QuantityType::Scalar, QuantityType::Scalar) => {
                Quantity::new(self.value.powf(rhs.value), QuantityType::Scalar)
            }
            _ => todo!(),
        }
    }

    /// Calculate the power of quantity and an integer.
    ///
    /// *Note: This function has not been implemented completely.*
    pub fn pow_int(&self, rhs: &Integer) -> Self {
        match &self.quantity_type {
            QuantityType::Scalar => {
                Quantity::new(self.value.powi(*rhs as i32), QuantityType::Scalar)
            }
            QuantityType::Length => todo!(),
            QuantityType::Area => todo!(),
            QuantityType::Volume => todo!(),
            QuantityType::Density => todo!(),
            QuantityType::Angle => todo!(),
            QuantityType::Weight => todo!(),
            QuantityType::Invalid => todo!(),
        }
    }
}

impl PartialOrd for Quantity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.quantity_type == other.quantity_type {
            self.value.partial_cmp(&other.value)
        } else {
            None
        }
    }
}

impl From<Scalar> for Quantity {
    fn from(value: Scalar) -> Self {
        Self::new(value, QuantityType::Scalar)
    }
}

impl From<Integer> for Quantity {
    fn from(value: Integer) -> Self {
        Self::new(value as Scalar, QuantityType::Scalar)
    }
}

impl Ty for Quantity {
    fn ty(&self) -> Type {
        Type::Quantity(self.quantity_type.clone())
    }
}

impl std::fmt::Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.quantity_type.base_unit())
    }
}

impl std::fmt::Debug for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.quantity_type, self.value)
    }
}
