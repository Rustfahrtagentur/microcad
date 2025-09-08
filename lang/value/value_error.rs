// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Value errors.

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
    CannotConvertToColor(Box<Tuple>),

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
    #[error("Tuple type mismatch: lhs={lhs}, rhs={rhs}")]
    TupleTypeMismatch {
        /// Left hand operand.
        lhs: Type,
        /// Right hand operand.
        rhs: Type,
    },

    /// Duplicate parameter
    #[error("Duplicate parameter: {0}")]
    DuplicateParameter(Identifier),

    /// Could not find identifier
    #[error("Identifier not found: {0}")]
    IdNotFound(Identifier),

    /// Expected a common type, e.g. for a [`ValueList`].
    #[error("Common type expected")]
    CommonTypeExpected,
}
