// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluation error

use crate::{eval::*, model::OutputType, parse::*, resolve::*, syntax::*, ty::*, value::*};
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

    /// Array elements have different types.
    #[error("Array elements have different types: {0}")]
    ArrayElementsDifferentTypes(TypeList),

    /// Symbol not found.
    #[error("Symbol {0} not found.")]
    SymbolNotFound(QualifiedName),

    /// Given symbol has not children which can be used.
    #[error("No symbols found to use in {0}")]
    NoSymbolsToUse(QualifiedName),

    /// Symbol was not expected to be found (e.g. `assert_invalid`).
    #[error("Unexpectedly found symbol {0}")]
    SymbolFound(QualifiedName),

    /// The symbol cannot be called, e.g. when it is a source file or a module.
    #[error("Symbol `{0}` cannot be called {1}.")]
    SymbolCannotBeCalled(QualifiedName, Box<SymbolDefinition>),

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

    /// A property of a value was not found.
    #[error("Property not found: {0}")]
    PropertyNotFound(Identifier),

    /// A property of a value was not found.
    #[error("Not a property id: {0}")]
    NoPropertyId(QualifiedName),

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

    /// Unexpected argument.
    #[error("Unexpected argument: {0}: {1}")]
    UnexpectedArgument(Identifier, Type),

    /// Assertion failed.
    #[error("Assertion failed: {0}")]
    AssertionFailed(String),

    /// Different type expected.
    #[error("Expected type `{expected}`, found type `{found}")]
    ExpectedType {
        /// Expected type.
        expected: Type,
        /// Found type.
        found: Type,
    },

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

    /// Unknown method.
    #[error("Unknown method `{0}`")]
    UnknownMethod(QualifiedName),

    /// Parser Error
    #[error("Parsing error {0}")]
    ParseError(#[from] ParseError),

    /// Statement is not supported in this context.
    #[error("{0} statement not available here")]
    StatementNotSupported(&'static str),

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
    #[error("Missing arguments: {0}")]
    MissingArguments(IdentifierList),

    /// Missing arguments
    #[error("Too many arguments: {0}")]
    TooManyArguments(IdentifierList),

    /// Builtin error
    #[error("Builtin error: {0}")]
    BuiltinError(String),

    /// Parameter not found by type in ParameterValueList
    #[error("Parameter not found by type '{0}'")]
    ParameterByTypeNotFound(Type),

    /// Trying to use multiplicity where it is not allowed
    #[error("Multiplicity not allowed '{0}'")]
    MultiplicityNotAllowed(IdentifierList),

    /// An error if you try to mix 2d and 3d geometries.
    #[error("Cannot mix 2d and 3d geometries")]
    CannotMixGeometry,

    /// A condition of an if statement is not a boolean
    #[error("If condition is not a boolean: {0}")]
    IfConditionIsNotBool(Value),

    /// Workbench didn't find a initialization routine matching the given arguments
    #[error("Workbench {0} cannot find initialization for those arguments")]
    NoInitializationFound(Identifier),

    /// Initializer missed to set a property from plan
    #[error("Workbench plan incomplete. Missing properties: {0}")]
    BuildingPlanIncomplete(IdentifierList),

    /// This errors happens if the expression is supposed to produce models but did not.
    #[error("This expression statement did not produce any model")]
    EmptyModelExpression,

    /// This error happens if the workbench produced a different output type.
    #[error("The {0} workbench produced a 2D output, but expected {2} output.")]
    WorkbenchInvalidOutput(WorkbenchKind, OutputType, OutputType),

    /// This error happens if the workbench produced a different output type.
    #[error("The {0} workbench will produce no {1} output.")]
    WorkbenchNoOutput(WorkbenchKind, OutputType),

    /// Unexpected source file in expression
    #[error("Unexpected source file {0} in expression")]
    InvalidSelfReference(Identifier),

    /// Resolve Error
    #[error("Resolve error {0}")]
    ResolveError(ResolveError),

    /// Unexpected source file in expression
    #[error("{0} is not operation.")]
    NotAnOperation(QualifiedName),

    /// Calling an operation on an empty geometry, e.g.: `{}.op()`.
    #[error("Calling operation on empty geometry")]
    OperationOnEmptyGeometry,

    /// Cannot call operation without workpiece, e.g. `op()`.
    #[error("Cannot call operation without workpiece.")]
    CannotCallOperationWithoutWorkpiece,

    /// Function missing return statement
    #[error("Missing return statement in {0}")]
    MissingReturn(QualifiedName),

    /// There is no model in this workbench
    #[error("Missing model in workbench")]
    NoModelInWorkbench,

    /// Found a symbol and a property with that name
    #[error("Found a symbol and a property with names {0} and {1}")]
    AmbiguousProperty(QualifiedName, Identifier),

    /// Assignment failed because value already has been initialized
    #[error("Value {0} already has been initialized with {1} (at line {2})")]
    ValueAlreadyInitialized(Identifier, Value, SrcRef),

    /// Assignment failed because left side is not an l-value
    #[error("Assignment failed because {0} is not an l-value")]
    NotAnLValue(Identifier),

    /// Found symbol but it's not visible to user
    #[error("Symbol {what} is private from within {within}")]
    SymbolIsPrivate {
        /// what was searched
        what: QualifiedName,
        /// where it was searched
        within: QualifiedName,
    },

    /// Found symbol but it's not visible to user
    #[error("Symbol {what} (aliased from {alias}) is private from within {within}")]
    SymbolBehindAliasIsPrivate {
        /// what was searched
        what: QualifiedName,
        /// the alias in between
        alias: QualifiedName,
        /// where it was searched
        within: QualifiedName,
    },
}

/// Result type of any evaluation.
pub type EvalResult<T> = std::result::Result<T, EvalError>;

impl From<ResolveError> for EvalError {
    fn from(value: ResolveError) -> Self {
        match value {
            ResolveError::SymbolNotFound(name) => EvalError::SymbolNotFound(name),
            other => EvalError::ResolveError(other),
        }
    }
}
