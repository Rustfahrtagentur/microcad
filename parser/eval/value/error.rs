use crate::{eval::*, r#type::*};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValueError {
    #[error("Invalid operator: {0}")]
    InvalidOperator(char),

    #[error("Tuple length mismatch for operator {operator}: lhs={lhs}, rhs={rhs}")]
    TupleLengthMismatchForOperator {
        operator: char,
        lhs: usize,
        rhs: usize,
    },

    #[error("Type cannot be a key in a map: {0}")]
    InvalidMapKeyType(Type),

    #[error("Cannot convert value {0} to {1}")]
    CannotConvert(Value, String),

    #[error("Cannot convert value into boolean: {0}")]
    CannotConvertToBool(Value),

    #[error("Cannot add unit to a value that has already a unit: {0}")]
    CannotAddUnitToValueWithUnit(Value),
}
