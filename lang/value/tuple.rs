// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unnamed tuple evaluation entity

use crate::{ty::*, value::*};

/// Unnamed tuple
#[derive(Clone, Debug, PartialEq)]
pub struct Tuple(ValueList);

impl Tuple {
    /// create a new unnamed tuple
    pub fn new(list: ValueList) -> Self {
        Self(list)
    }

    /// evaluate the given operation
    pub fn binary_op(
        self,
        rhs: Self,
        op: char,
        f: impl Fn(Value, Value) -> ValueResult,
    ) -> std::result::Result<Self, ValueError> {
        if self.0.len() != rhs.0.len() {
            return Err(ValueError::TupleLengthMismatchForOperator {
                operator: op,
                lhs: self.0.len(),
                rhs: rhs.0.len(),
            });
        }
        let mut result = Vec::new();
        for (l, r) in self.0.iter().zip(rhs.0.iter()) {
            let add_result = f(l.clone(), r.clone())?;
            result.push(add_result);
        }

        Ok(Tuple(ValueList::new(result)))
    }
}

impl From<ValueList> for Tuple {
    fn from(value: ValueList) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({})",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl crate::ty::Ty for Tuple {
    fn ty(&self) -> Type {
        Type::Tuple(TupleType(self.0.iter().map(|v| v.ty().clone()).collect()))
    }
}

impl std::ops::Add for Tuple {
    type Output = std::result::Result<Tuple, ValueError>;

    fn add(self, rhs: Self) -> Self::Output {
        self.binary_op(rhs, '+', |lhs, rhs| lhs + rhs)
    }
}

impl std::ops::Sub for Tuple {
    type Output = std::result::Result<Tuple, ValueError>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.binary_op(rhs, '-', |lhs, rhs| lhs - rhs)
    }
}
