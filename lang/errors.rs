// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parser errors

use crate::parse::*;
use thiserror::Error;

/// Parsing errors
#[derive(Debug, Error)]
pub enum ParseError {
    /// Expected identifier
    #[error("Expected identifier")]
    ExpectedIdentifier,

    /// Invalid use statement
    #[error("")]
    InvalidUseStatement,

    /// Error parsing floating point literal
    #[error("Error parsing floating point literal: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),

    /// Error parsing integer literal
    #[error("Error parsing integer literal: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    /// Error parsing color literal
    #[error("Error parsing color literal: {0}")]
    ParseColorError(String),

    /// Unknown unit
    #[error("Unknown unit: {0}")]
    UnknownUnit(String),

    /// Unexpected token
    #[error("Unexpected token")]
    UnexpectedToken,

    /// Tuple expression contains both named and positional arguments
    #[error("Tuple expression contains both named and positional arguments")]
    MixedTupleArguments,

    /// Duplicate named argument
    #[error("Duplicate named argument: {0}")]
    DuplicateNamedArgument(Identifier),

    /// Positional argument after named argument
    #[error("Positional argument after named argument")]
    PositionalArgumentAfterNamed,

    /// Empty tuple expression
    #[error("Empty tuple expression")]
    EmptyTupleExpression,

    /// Missing type or value for definition parameter
    #[error("Missing type or value for definition parameter: {0}")]
    ParameterMissingTypeOrValue(Identifier),

    /// Duplicate parameter
    #[error("Duplicate parameter: {0}")]
    DuplicateParameter(Identifier),

    /// Duplicate argument
    #[error("Duplicate argument: {0}")]
    DuplicateCallArgument(Identifier),

    /// Invalid map key type
    #[error("Invalid map key type: {0}")]
    InvalidMapKeyType(String),

    /// Duplicated field name in map
    #[error("Duplicated field name in map: {0}")]
    DuplicatedMapField(Identifier),

    /// Duplicate identifier
    #[error("Duplicate identifier: {0}")]
    DuplicateIdentifier(Identifier),

    /// Missing format expression
    #[error("Missing format expression")]
    MissingFormatExpression,

    /// Statement between two init statements
    #[error("Statement between two init statements")]
    StatementBetweenModuleInit,

    /// A module has both a parameter list and initializer
    #[error("Module has both a parameter list and initializer")]
    BothParameterListAndInitializer,
}

/// Result with parse error
pub type ParseResult<T> = Result<T, ParseError>;
