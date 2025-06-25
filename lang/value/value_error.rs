// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::value::{error::QuantityError, *};
use thiserror::Error;

/// Value error
#[derive(Debug, Error)]
pub enum ValueError {
    /// Invalid operator
    #[error("Invalid operator: {0}")]
    InvalidOperator(String),

    /// Quantity Error.
    #[error("Quantity error: {0}")]
    QuantityError(#[from] QuantityError),

    /// Cannot convert to color.
    #[error("Cannot convert named tuple to color: {0}")]
    CannotConvertToColor(Tuple),

    /// Type cannot be a key in a map
    #[error("Type cannot be a key in a map: {0}")]
    InvalidMapKeyType(Type),

    /// Cannot add unit to a value that has already a unit
    #[error("Cannot add unit to a value that has already a unit: {0}")]
    CannotAddUnitToValueWithUnit(Value),

    /// Cannot convert value
    #[error("Cannot convert value {0} to {1}")]
    CannotConvert(Value, String),

    /// Cannot convert value into boolean
    #[error("Cannot convert value into boolean: {0}")]
    CannotConvertToBool(Value),

    /// Cannot concat two vec with different types
    #[error("Cannot concat two vec with different types {0} and {1}")]
    CannotCombineVecOfDifferentType(Type, Type),

    /// Tuple length mismatch
    #[error("Tuple length mismatch for operator {operator}: lhs={lhs}, rhs={rhs}")]
    TupleLengthMismatchForOperator {
        /// Operator
        operator: char,
        /// Left hand operand
        lhs: usize,
        /// Right hand operand
        rhs: usize,
    },

    /// Duplicate parameter
    #[error("Duplicate parameter: {0}")]
    DuplicateParameter(Identifier),
}
