// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Boolean operations

#[derive(Debug)]
/// Boolean operations
pub enum BooleanOp {
    /// Computes the union R = P ∪ Q
    Union,
    /// computes the difference R = P ∖ Q
    Subtract,
    /// computes the complement R=P̅
    Complement,
    /// computes the intersection R = P ∩ Q
    Intersect,
}

use geo::OpType;

impl From<BooleanOp> for OpType {
    fn from(op: BooleanOp) -> Self {
        Self::from(&op)
    }
}

impl From<&BooleanOp> for OpType {
    fn from(op: &BooleanOp) -> Self {
        match op {
            BooleanOp::Subtract => OpType::Difference,
            BooleanOp::Union => OpType::Union,
            BooleanOp::Intersect => OpType::Intersection,
            BooleanOp::Complement => OpType::Xor,
        }
    }
}
