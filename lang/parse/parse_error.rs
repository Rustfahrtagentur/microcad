// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parser errors

use crate::{parse::*, ty::*};
use thiserror::Error;

/// Parsing errors
#[derive(Debug, Error)]
pub enum ParseError {
    /// Expected identifier
    #[error("Expected identifier")]
    ExpectedIdentifier,

    /// Error parsing floating point literal
    #[error("Error parsing floating point literal: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),

    /// Error parsing integer literal
    #[error("Error parsing integer literal: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    /// Parser rule error
    #[error("Cannot parse rule: {0:?}")]
    RuleError(Box<crate::parser::Rule>),

    /// IO Error
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),

    /// Error in pest parser
    #[error("Parser error: {0}")]
    Parser(#[from] Box<pest::error::Error<crate::parser::Rule>>),

    /// Error parsing color literal
    #[error("Error parsing color from hex: {0}")]
    ParseColorFromHex(String),

    /// Unknown color name
    #[error("Unknown color: {0}")]
    UnknownColorName(String),

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
    DuplicateArgument(Identifier),

    /// Invalid map key type
    #[error("Invalid map key type: {0}")]
    InvalidMapKeyType(String),

    /// Duplicated type name in map
    #[error("Duplicated type name in map: {0}")]
    DuplicatedMapType(Identifier),

    /// Duplicate identifier
    #[error("Duplicate identifier: {0}")]
    DuplicateIdentifier(Identifier),

    /// Duplicate identifier in tuple
    #[error("Duplicate identifier in tuple: {0}")]
    DuplicateTupleIdentifier(Identifier),

    /// Duplicate unnamed type in tuple
    #[error("Duplicate unnamed type in tuple: {0}")]
    DuplicateTupleType(Type),

    /// Missing format expression
    #[error("Missing format expression")]
    MissingFormatExpression,

    /// Statement between two init statements
    #[error("Statement between two init statements")]
    StatementBetweenInit,

    /// Loading of a source file failed
    #[error("Loading of source file {0:?} failed")]
    LoadSource(std::path::PathBuf),

    /// Grammar rule error
    #[error("Grammar rule error {0}")]
    GrammarRuleError(String),

    /// Grammar rule error
    #[error("Invalid qualified name '{0}'")]
    InvalidQualifiedName(String),

    /// Grammar rule error
    #[error("Invalid identifier '{0}'")]
    InvalidIdentifier(String),

    /// Qualified name cannot be converted into an Id
    #[error("Qualified name {0} cannot be converted into an Id")]
    QualifiedNameIsNoId(QualifiedName),

    /// Statement not allowed within workbenches
    #[error("Statement not allowed within workbenches")]
    IllegalWorkbenchStatement,

    /// Code Between initializers
    #[error("Code between initializers is not allowed")]
    CodeBetweenInitializers,

    /// Statement not allowed prior initializers
    #[error("Statement not allowed prior initializers")]
    StatementNotAllowedPriorInitializers,
}

/// Result with parse error
pub type ParseResult<T> = Result<T, ParseError>;
