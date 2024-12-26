// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unnamed record evaluation entity

use crate::{eval::*, parse::*, r#type::*, src_ref::*};

/// Unnamed tuple
#[derive(Clone, Debug, PartialEq)]
pub struct UnnamedRecord(ValueList);

impl UnnamedRecord {
    /// create a new unnamed record
    pub fn new(list: ValueList) -> Self {
        Self(list)
    }

    /// evaluate the given operation
    pub fn binary_op(
        self,
        rhs: Self,
        op: char,
        f: impl Fn(Value, Value) -> ValueResult,
    ) -> std::result::Result<Self, EvalError> {
        if self.0.len() != rhs.0.len() {
            return Err(EvalError::RecordLengthMismatchForOperator {
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

        Ok(UnnamedRecord(ValueList::new(
            result,
            SrcRef::merge(&self, &rhs),
        )))
    }
}

impl SrcReferrer for UnnamedRecord {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

impl From<ValueList> for UnnamedRecord {
    fn from(value: ValueList) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for UnnamedRecord {
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

impl Ty for UnnamedRecord {
    fn ty(&self) -> Type {
        Type::UnnamedRecord(UnnamedRecordType(
            self.0.iter().map(|v| v.ty().clone()).collect(),
        ))
    }
}

impl std::ops::Add for UnnamedRecord {
    type Output = std::result::Result<UnnamedRecord, EvalError>;

    fn add(self, rhs: Self) -> Self::Output {
        self.binary_op(rhs, '+', |lhs, rhs| lhs + rhs)
    }
}

impl std::ops::Sub for UnnamedRecord {
    type Output = std::result::Result<UnnamedRecord, EvalError>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.binary_op(rhs, '-', |lhs, rhs| lhs - rhs)
    }
}
