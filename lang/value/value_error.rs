use crate::value::*;
use thiserror::Error;

/// Value error
#[derive(Debug, Error)]
pub enum ValueError {
    /// Invalid operator
    #[error("Invalid operator: {0}")]
    InvalidOperator(String),

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

    /// Cannot compare two values
    #[error("Incompatible types of {0} and {1} for operation {2}")]
    BinaryOpNotAvailable(Value, Value, String),

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

    /// Missing arguments
    #[error("Missing arguments: {0}")]
    MissingArguments(ParameterValueList),

    /// Duplicate parameter
    #[error("Duplicate parameter: {0}")]
    DuplicateParameter(Identifier),
}
