// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluation error

use crate::{
    eval::*,
    parse::{Identifier, QualifiedName},
    r#type::*,
};
use microcad_core::Id;
use thiserror::Error;

/// Evaluation error
#[derive(Debug, Error)]
pub enum EvalError {
    /// Custom evaluation error
    #[error("{0}")]
    CustomError(String),

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

    /// Parameter type mismatch
    #[error("Type mismatch for parameter: expected {expected}, got {found}")]
    ParameterTypeMismatch {
        /// Parameter name
        name: Identifier,
        /// expected type
        expected: Type,
        /// found type
        found: Type,
    },

    /// Return type mismatch
    #[error("Return type mismatch: expected {expected}, got {found}")]
    ReturnTypeMismatch {
        /// Parameter name
        name: Identifier,
        /// expected type
        expected: Type,
        /// found type
        found: Type,
    },

    /// Expected a namespace symbol
    #[error("Namespace symbol expected, got {0}")]
    NamespaceSymbolExpected(Symbol),

    /// Cannot evaluate to type
    #[error("Cannot evaluate to type: {0}")]
    EvaluateToTypeError(Type),

    /// Cannot use symbol
    #[error("Cannot use {0}")]
    CannotUse(Symbol),

    /// Unknown qualified name
    #[error("Unknown qualified name: {0}")]
    UnknownQualifiedName(Id),

    /// Unknown method
    #[error("Unknown method: {0}")]
    UnknownMethod(Identifier),

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

    /// No matching initializer for module definition
    #[error("No matching initializer for module definition `{0}`")]
    NoMatchingInitializer(Identifier),

    /// Multiple matching Initializers for module definition
    #[error("Multiple matching initializer for module definition `{0}`")]
    MultipleMatchingInitializer(Identifier),

    /// Expected range in for loop
    #[error("Expected range in for loop, got {0}")]
    ExpectedRangeInForLoop(Type),

    /// Expected iterable, a list or a range
    #[error("Expected iterable, got {0}")]
    ExpectedIterable(Type),

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

    /// Cannot nest symbol
    #[error("Cannot nest symbol: {0}")]
    CannotNestSymbol(Symbol),

    /// Cannot nest item
    #[error("Cannot nest item: {0}")]
    CannotNestItem(crate::parse::NestedItem),

    /// Missing arguments
    #[error("Missing arguments: {0:?}")]
    MissingArguments(ParameterValueList),

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

    /// Assertion failed
    #[error("Assertion failed: {0} with ")]
    AssertionFailedWithCondition(String, String),

    /// Ambiguous symbol
    #[error("Ambiguous symbol: {0}")]
    AmbiguousSymbol(Symbol),

    /// Unknown field, e.g. node.field, where node.field
    #[error("Unknown field: {0}")]
    UnknownField(Identifier),

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

    /// Invalid node marker
    #[error("Invalid node marker: {0}")]
    InvalidNodeMarker(Identifier),

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

    /// Symbol is not callable
    #[error("Symbol is not callable: {0}")]
    SymbolNotCallable(Symbol),

    /// Cannot continue evaluation after error limit has been reached
    #[error("Error limit reached: Stopped evaluation after {0} errors")]
    ErrorLimitReached(u32),

    /// Unexpected empty stack
    #[error("Unexpected empty stack")]
    UnexpectedEmptyStack,

    /// Tuple item not found
    #[error("Tuple item not found {0}")]
    TupleItemNotFound(Identifier),

    /// Cannot get argument
    #[error("Cannot get argument {0}")]
    CannotGetArgument(&'static str),

    /// Grammar rule error
    #[error("Grammar rule error")]
    GrammarRuleError(String),

    /// Named tuple element access error
    #[error("Named tuple element access error")]
    NamedTupleElementAccess(Value),

    /// Wrong parameters in call
    #[error("Wrong parameters in call to module {0}")]
    WrongModuleParameters(QualifiedName),

    /// Missed call
    #[error("Missed call")]
    MissedCall,

    /// StackUnderflow
    #[error("Stack underflow")]
    StackUnderflow,
}

/// Result type of any evaluation
pub type EvalResult<T> = std::result::Result<T, EvalError>;
