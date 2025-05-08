// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Number literal syntax element

use crate::{src_ref::*, syntax::*, ty::*, value::Value};

/// Number literal
#[derive(Debug, Clone, PartialEq)]
pub struct NumberLiteral(pub f64, pub Unit, pub SrcRef);

impl NumberLiteral {
    /// Create from usize value
    #[cfg(test)]
    pub fn from_usize(value: usize) -> Self {
        Self(value as f64, Unit::None, SrcRef(None))
    }

    /// Create from integer value
    #[cfg(test)]
    pub fn from_int(value: i64) -> Self {
        Self(value as f64, Unit::None, SrcRef(None))
    }

    /// Returns the actual value of the literal
    pub fn normalized_value(&self) -> f64 {
        self.1.normalize(self.0)
    }

    /// return unit
    pub fn unit(&self) -> Unit {
        self.1
    }

    /// Return value for number literal
    pub fn value(&self) -> Value {
        match self.1.ty() {
            Type::Scalar => Value::Scalar(self.normalized_value()),
            Type::Angle => Value::Angle(self.normalized_value()),
            Type::Length => Value::Length(self.normalized_value()),
            Type::Weight => Value::Weight(self.normalized_value()),
            Type::Area => Value::Area(self.normalized_value()),
            Type::Volume => Value::Volume(self.normalized_value()),
            _ => unreachable!(),
        }
    }
}

impl crate::ty::Ty for NumberLiteral {
    fn ty(&self) -> Type {
        self.1.ty()
    }
}

impl SrcReferrer for NumberLiteral {
    fn src_ref(&self) -> literal::SrcRef {
        self.2.clone()
    }
}

impl std::fmt::Display for NumberLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl From<NumberLiteral> for Value {
    fn from(literal: NumberLiteral) -> Self {
        literal.value()
    }
}
