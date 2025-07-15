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
    #[error("Type mismatch for `{id}`: expected {expected}, got {found}")]
    TypeMismatch {
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
        args: ArgumentValueList,
        /// Expected number of arguments
        expected: usize,
        /// Found number of arguments
        found: usize,
    },

    /// Called assertion
    #[error("assert called with wrong number of arguments.")]
    AssertWrongSignature(ArgumentValueList),

    /// Invalid argument type.
    #[error("Invalid argument type: {0}")]
    InvalidArgumentType(Type),

    /// Cannot nest item.
    #[error("Cannot nest item: {0}")]
    CannotNestItem(NestedItem),

    /// Unexpected argument.
    #[error("Unexpected argument: {0}: {1}")]
    UnexpectedArgument(Identifier, Type),

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
    #[error("External symbol `{0}` not found")]
    ExternalSymbolNotFound(QualifiedName),

    /// Path of external file is unknown.
    #[error("External path `{0}` not found")]
    ExternalPathNotFound(std::path::PathBuf),

    /// Can't find a project file by hash.
    #[error("Could not find a file with hash {0}")]
    UnknownHash(u64),

    /// Unknown method.
    #[error("Unknown method `{0}`")]
    UnknownMethod(Identifier),

    /// Can't find a project file by it's path.
    #[error("Could not find a file with path {0}")]
    UnknownPath(std::path::PathBuf),

    /// Can't find a project file by it's qualified name.
    #[error("Parsing error {0}")]
    ParseError(#[from] ParseError),

    /// Statement is not supported in this context.
    #[error("Statement not supported: {0}")]
    StatementNotSupported(Box<Statement>),

    /// Properties are not initialized.
    #[error("Properties have not been initialized: {0}")]
    UninitializedProperties(IdentifierList),

    /// Unexpected element within expression.
    #[error("Unexpected {0} {1} within expression")]
    UnexpectedNested(&'static str, Identifier),

    /// No variables allowed in definition
    #[error("No variables allowed in {0}")]
    NoVariablesAllowedIn(&'static str),

    /// Error when evaluating attributes.
    #[error("Attribute error: {0}")]
    AttributeError(#[from] AttributeError),

    /// Missing arguments
    #[error("Missing arguments: {0:?}")]
    MissingArguments(Vec<Identifier>),

    /// Missing arguments
    #[error("Too many arguments: {0:?}")]
    TooManyArguments(Vec<Identifier>),

    /// Builtin error
    #[error("Builtin error: {0}")]
    BuiltinError(String),

    /// Parameter not found by type in ParameterValueList
    #[error("Parameter not found by type '{0}'")]
    ParameterByTypeNotFound(Type),

    /// Trying to use multiplicity where it is not allowed
    #[error("Multiplicity not allowed '{0:?}'")]
    MultiplicityNotAllowed(std::collections::HashSet<Identifier>),

    /// An error if you try to mix 2d and 3d geometries.
    #[error("Cannot mix 2d and 3d geometries")]
    CannotMixGeometry,

    /// A condition of an if statement is not a boolean
    #[error("If condition is not a boolean: {0}")]
    IfConditionIsNotBool(Value),

    /// Workbench didn't find a initialization routine matching the given arguments
    #[error("Workbench {0} cannot find initialization for those arguments")]
    NoInitializationFound(Identifier),
}

/// Result type of any evaluation.
pub type EvalResult<T> = std::result::Result<T, EvalError>;
