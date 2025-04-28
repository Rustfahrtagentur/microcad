// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluation error

use crate::{parse::*, src_ref::SrcRef, syntax::*, ty::*, value::*, Id};
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

    /// Invalid type
    #[error("Invalid type: {0}")]
    InvalidI(Type),

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

    /// Parameter names must be unique
    #[error("Duplicated parameter: {0}")]
    DuplicatedParameter(Id),

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

    /// Cannot evaluate to type
    #[error("Cannot evaluate to type: {0}")]
    EvaluateToTypeError(Type),

    /// Unknown qualified name
    #[error("Unknown qualified name: {0}")]
    UnknownQualifiedName(Id),

    /// Unknown method
    #[error("Unknown method: {0}")]
    UnknownMethod(Identifier),

    /// Elements of list have different types
    #[error("Elements of list have different types: {0}")]
    ListElementsDifferentTypes(TypeList),

    /// Function call missing argument
    #[error("Function call missing argument: {0}")]
    FunctionCallMissingArgument(Id),

    /// Function must return a value
    #[error("Function must return a value")]
    FunctionCallMissingReturn,

    /// Symbol not found
    #[error("Symbol {0} not found.")]
    SymbolNotFound(QualifiedName),

    /// Symbol not found (retry to load from external)
    #[error("Symbol {0} must be loaded from {1:?}")]
    SymbolMustBeLoaded(QualifiedName, std::path::PathBuf),

    /// Symbol is not a value
    #[error("Symbol does not contain a value: {0}")]
    SymbolIsNotAValue(QualifiedName),

    /// Given symbol has not children which can be used
    #[error("No symbol found to use in {0}")]
    NoSymbolFound(QualifiedName),

    /// Symbol was not expected to be found (e.g. assert_invalid)
    #[error("Symbol {0} found unexpected")]
    SymbolFound(QualifiedName),

    /// Local symbol not found
    #[error("Local symbol not found: {0}")]
    LocalNotFound(Id),

    /// Expression is neither a valid name for a symbol nor local variable
    #[error("'{0}' is neither a valid name for a symbol nor local variable")]
    NotAName(SrcRef),

    /// Lookup of a name failed
    #[error("Lookup of name {0} failed")]
    LookUpFailed(Expression),

    /// No matching initializer for module definition
    #[error("No matching initializer for module definition `{0}`")]
    NoMatchingInitializer(Identifier),

    /// Multiple matching Initializers for module definition
    #[error("Multiple matching initializer for module definition `{0}`")]
    MultipleMatchingInitializer(Identifier),

    /// A property of a value was not found
    #[error("Property not found: {0}")]
    PropertyNotFound(Identifier),

    /// Expected range in for loop
    #[error("Expected range in for loop, got {0}")]
    ExpectedRangeInForLoop(Type),

    /// Expected iterable, a list or a range
    #[error("Expected iterable, got {0}")]
    ExpectedIterable(Type),

    /// Argument count mismatch
    #[error("Argument count mismatch: expected {expected}, got {found} in {args}")]
    ArgumentCountMismatch {
        /// Argument list including the error
        args: CallArgumentList,
        /// Expected number of arguments
        expected: usize,
        /// Found number of arguments
        found: usize,
    },

    /// Found ambiguous symbol
    #[error("Ambiguous symbol {ambiguous} might be one of {others:?}")]
    AmbiguousSymbol {
        /// ambiguous symbol
        ambiguous: QualifiedName,
        /// local symbol that matches
        others: Vec<QualifiedName>,
    },

    /// Invalid argument type
    #[error("Invalid argument type: {0}")]
    InvalidArgumentType(Type),

    /// Expected module
    #[error("Expected module: {0}")]
    ExpectedModule(Id),

    /// Cannot nest item
    #[error("Cannot nest item: {0}")]
    CannotNestItem(NestedItem),

    /// Parameter missing type or value
    #[error("Parameter missing type or value: {0}")]
    ParameterMissingTypeOrValue(Id),

    /// Unexpected argument
    #[error("Unexpected argument: {0}")]
    UnexpectedArgument(Id),

    /// Duplicate call argument
    #[error("Duplicate call argument: {0}")]
    DuplicateCallArgument(Id),

    /// Assertion failed
    #[error("Assertion failed: {0}")]
    AssertionFailed(String),

    /// Unknown field, e.g. node.field, where node.field
    #[error("Unknown field: {0}")]
    UnknownField(Identifier),

    /// Invalid node marker
    #[error("Invalid node marker: {0}")]
    InvalidNodeMarker(Identifier),

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

    /// Wrong parameters in call
    #[error("Wrong parameters in call to module {0}")]
    WrongModuleParameters(QualifiedName),

    /// Missed call
    #[error("Missed call")]
    MissedCall,

    /// Value Error
    #[error("Value Error: {0}")]
    ValueError(#[from] ValueError),

    /// Name of external symbol is unknown
    #[error("External symbol {0} not found")]
    ExternalSymbolNotFound(QualifiedName),

    /// Path of external file is unknown
    #[error("External path '{0}' not found")]
    ExternalPathNotFound(std::path::PathBuf),

    /// Can't find a project file by hash
    #[error("Could not find a file with hash {0}")]
    UnknownHash(u64),

    /// Can't find a project file by it's path
    #[error("Could not find a file with path {0}")]
    UnknownPath(std::path::PathBuf),

    /// Can't find a project file by it's qualified name
    #[error("Could not find a file with name {0}")]
    UnknownName(QualifiedName),

    /// Found two external source files which point to the same namespace
    #[error("Ambiguous external files {0} and {1}")]
    AmbiguousExternal(std::path::PathBuf, std::path::PathBuf),

    /// Can't find a project file by it's qualified name
    #[error("Parsing error {0}")]
    ParseError(#[from] ParseError),
}

/// Result type of any evaluation
pub type EvalResult<T> = std::result::Result<T, EvalError>;
