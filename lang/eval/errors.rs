// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, r#type::*};
use microcad_core::Id;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EvalError {
    /// Unknown error
    #[error("Unknown error")]
    Unknown,

    /// Invalid operator
    #[error("Invalid operator: {0}")]
    InvalidOperator(String),

    /// Incompatible types for addition
    #[error("Incompatible types {0} and {1} for addition")]
    AddIncompatibleTypes(Type, Type),

    /// Incompatible types for subtraction
    #[error("Incompatible types {0} and {1} for subtraction")]
    SubIncompatibleTypes(Type, Type),

    /// Incompatible types for multiplication
    #[error("Incompatible types {0} and {1} for multiplication")]
    MulIncompatibleTypes(Type, Type),

    /// Incompatible types for division
    #[error("Incompatible types {0} and {1} for division")]
    DivIncompatibleTypes(Type, Type),

    /// Invalid type
    #[error("Invalid type: {0}")]
    InvalidType(Type),

    /// List index out of bounds
    #[error("List index out of bounds: {index} >= {len}")]
    ListIndexOutOfBounds {
        /// wrong index
        index: usize,
        /// length of list
        len: usize,
    },

    /// Type mismatch
    #[error("Type mismatch: expected {expected}, got {found}")]
    TypeMismatch {
        /// expected type
        expected: Type,
        /// found type
        found: Type,
    },

    /// Cannot evaluate to type
    #[error("Cannot evaluate to type: {0}")]
    EvaluateToTypeError(Type),

    /// Unknown qualified name
    #[error("Unknown qualified name: {0}")]
    UnknownQualifiedName(Id),

    /// Unknown method
    #[error("Unknown method: {0}")]
    UnknownMethod(Id),

    /// Elements of list have different types
    #[error("Elements of list have different types")]
    ListElementsDifferentTypes,

    /// Function call missing argument
    #[error("Function call missing argument: {0}")]
    FunctionCallMissingArgument(Id),

    /// Function must return a value
    #[error("Function must return a value")]
    FunctionCallMissingReturn,

    /// Symbol not found
    #[error("Symbol not found: {0}")]
    SymbolNotFound(Id),

    /// Argument count mismatch
    #[error("Argument count mismatch: expected {expected}, got {found}")]
    ArgumentCountMismatch {
        /// Expected number of arguments
        expected: usize,
        /// Found number of arguments
        found: usize,
    },

    /// Invalid argument type
    #[error("Invalid argument type: {0}")]
    InvalidArgumentType(Type),

    /// Expected module
    #[error("Expected module: {0}")]
    ExpectedModule(Id),

    /// Cannot nest function call
    #[error("Cannot nest function call")]
    CannotNestFunctionCall,

    /// Missing arguments
    #[error("Missing arguments: {0:?}")]
    MissingArguments(Vec<Id>),

    /// Parameter type mismatch
    #[error("Parameter type mismatch: {0} expected {1}, got {2}")]
    ParameterTypeMismatch(Id, Type, Type),

    /// Parameter missing type or value
    #[error("Parameter missing type or value: {0}")]
    ParameterMissingTypeOrValue(Id),

    /// Unexpected argument
    #[error("Unexpected argument: {0}")]
    UnexpectedArgument(Id),

    /// Duplicate call argument
    #[error("Duplicate call argument: {0}")]
    DuplicateCallArgument(Id),

    /// Duplicate parameter
    #[error("Duplicate parameter: {0}")]
    DuplicateParameter(Id),

    /// Assertion failed
    #[error("Assertion failed: {0}")]
    AssertionFailed(String),

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

    /// Type cannot be a key in a map
    #[error("Type cannot be a key in a map: {0}")]
    InvalidMapKeyType(Type),

    /// Cannot convert value
    #[error("Cannot convert value {0} to {1}")]
    CannotConvert(Value, String),

    /// Cannot convert value into boolean
    #[error("Cannot convert value into boolean: {0}")]
    CannotConvertToBool(Value),

    /// Cannot add unit to a value that has already a unit
    #[error("Cannot add unit to a value that has already a unit: {0}")]
    CannotAddUnitToValueWithUnit(Value),

    /// Cannot concat two vec with different types
    #[error("Cannot concat two vec with different types {0} and {1}")]
    CannotCombineVecOfDifferentType(Type, Type),
}
