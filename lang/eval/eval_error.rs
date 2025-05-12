// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluation error

use crate::{eval::*, parse::*, resolve::*, src_ref::SrcRef, syntax::*, ty::*, value::*};
use thiserror::Error;

/// Evaluation error.
#[derive(Debug, Error)]
pub enum EvalError {
    /// Can't find a project file by it's qualified name.
    #[error("Not implemented: {0}")]
    Todo(String),

    /// List index out of bounds.
    #[error("List index out of bounds: {index} >= {len}")]
    ListIndexOutOfBounds {
        /// Wrong index
        index: usize,
        /// Length of list
        len: usize,
    },

    /// Parameter type mismatch.
    #[error("Type mismatch for parameter `{id}`: expected {expected}, got {found}")]
    ParameterTypeMismatch {
        /// Parameter name
        id: Identifier,
        /// Expected type
        expected: Type,
        /// Found type
        found: Type,
    },

    /// Elements of list have different types.
    #[error("Elements of list have different types: {0}")]
    ListElementsDifferentTypes(TypeList),

    /// Symbol not found.
    #[error("Symbol {0} not found.")]
    SymbolNotFound(QualifiedName),

    /// Symbol not found (retry to load from external).
    #[error("Symbol {0} must be loaded from {1}")]
    SymbolMustBeLoaded(QualifiedName, std::path::PathBuf),

    /// Given symbol has not children which can be used.
    #[error("No symbols found to use in {0}")]
    NoSymbolsToUse(QualifiedName),

    /// Symbol was not expected to be found (e.g. `assert_invalid`).
    #[error("Unexpectedly found symbol {0}")]
    SymbolFound(QualifiedName),

    /// Found ambiguous symbols.
    #[error("Ambiguous symbol {ambiguous} might be one of the following:\n{others}")]
    AmbiguousSymbol {
        /// Searched name
        ambiguous: QualifiedName,
        /// Symbols which matches the name
        others: Symbols,
    },

    /// Local Symbol not found.
    #[error("Local symbol not found: {0}")]
    LocalNotFound(Identifier),

    /// Expression is neither a valid name for a symbol nor local variable.
    #[error("'{0}' is neither a valid name for a symbol nor local variable")]
    NotAName(SrcRef),

    /// A property of a value was not found.
    #[error("Property not found: {0}")]
    PropertyNotFound(Identifier),

    /// Expected iterable, a list or a range.
    #[error("Expected iterable, got {0}")]
    ExpectedIterable(Type),

    /// Argument count mismatch.
    #[error("Argument count mismatch: expected {expected}, got {found} in {args}")]
    ArgumentCountMismatch {
        /// Argument list including the error
        args: CallArgumentValueList,
        /// Expected number of arguments
        expected: usize,
        /// Found number of arguments
        found: usize,
    },

    /// Invalid argument type.
    #[error("Invalid argument type: {0}")]
    InvalidArgumentType(CallArgumentValue),

    /// Cannot nest item.
    #[error("Cannot nest item: {0}")]
    CannotNestItem(NestedItem),

    /// Unexpected argument.
    #[error("Unexpected argument: {0}")]
    UnexpectedArgument(Identifier),

    /// Assertion failed.
    #[error("Assertion failed: {0}")]
    AssertionFailed(String),

    /// Cannot continue evaluation after error limit has been reached.
    #[error("Error limit reached: Stopped evaluation after {0} errors")]
    ErrorLimitReached(u32),

    /// No locals  available on stack.
    #[error("Local stack needed to store {0}")]
    LocalStackEmpty(Identifier),

    /// Unexpected stack frame type
    #[error("Unexpected stack frame of type '{1}' cannot store {0}")]
    WrongStackFrame(Identifier, &'static str),

    /// Value Error.
    #[error("Value Error: {0}")]
    ValueError(#[from] ValueError),

    /// Name of external symbol is unknown.
    #[error("External symbol {0} not found")]
    ExternalSymbolNotFound(QualifiedName),

    /// Path of external file is unknown.
    #[error("External path '{0}' not found")]
    ExternalPathNotFound(std::path::PathBuf),

    /// Can't find a project file by hash.
    #[error("Could not find a file with hash {0}")]
    UnknownHash(u64),

    /// Can't find a project file by it's path.
    #[error("Could not find a file with path {0}")]
    UnknownPath(std::path::PathBuf),

    /// Can't find a project file by it's qualified name.
    #[error("Parsing error {0}")]
    ParseError(#[from] ParseError),

    /// Statement is not supported in this context.
    #[error("Statement not supported: {0}")]
    StatementNotSupported(Statement),

    /// Properties are not initialized.
    #[error("Properties have not been initialized: {0:?}")]
    UninitializedProperties(IdentifierList),

    /// Unexpected element within expression.
    #[error("Unexpected {0} {1} within expression")]
    UnexpectedNested(&'static str, Identifier),

    /// No variables allowed in namespaces
    #[error("No variables allowed in {0}")]
    NoVariablesAllowedIn(&'static str),

    /// Parameter with that id could not be found in function signature
    #[error("Parameter '{0}' could not be found in function signature.")]
    ParameterNotFound(Identifier),

    /// Parameter is function signature but not given
    #[error("Missing parameter(s) {0}")]
    MissingParameter(ParameterValueList),

    /// Parameter is function signature but not given
    #[error("Missing parameter(s) {0}")]
    AmbiguousArgument(ParameterValueList),

    /// Parameter is function signature but not given
    #[error("No matching init found in {0} for call {1}")]
    NoMatchingInit(Identifier, CallArgumentValueList),
}

/// Result type of any evaluation.
pub type EvalResult<T> = std::result::Result<T, EvalError>;
