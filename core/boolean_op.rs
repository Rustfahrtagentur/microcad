// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Boolean operations

#[derive(Debug)]
/// Boolean operations
pub enum BooleanOp {
    /// Computes the union R = P ∪ Q
    Union,
    /// computes the difference R = P ∖ Q
    Difference,
    /// computes the complement R=P̅
    Complement,
    /// computes the intersection R = P ∩ Q
    Intersection,
}

use geo::OpType;

impl From<BooleanOp> for OpType {
    fn from(op: BooleanOp) -> Self {
        match op {
            BooleanOp::Difference => OpType::Difference,
            BooleanOp::Union => OpType::Union,
            BooleanOp::Intersection => OpType::Intersection,
            BooleanOp::Complement => OpType::Xor,
        }
    }
}

impl From<&BooleanOp> for OpType {
    fn from(op: &BooleanOp) -> Self {
        match op {
            BooleanOp::Difference => OpType::Difference,
            BooleanOp::Union => OpType::Union,
            BooleanOp::Intersection => OpType::Intersection,
            BooleanOp::Complement => OpType::Xor,
        }
    }
}
