// Copyright © 2024 The µcad authors <info@ucad.xyz>
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
    #[error("Error parsing color literal: {0}")]
    ParseColorError(String),

    /// Unknown unit
    #[error("Unknown unit: {0}")]
    UnknownUnit(String),

    /// Unexpected token
    #[error("Unexpected token")]
    UnexpectedToken,

    /// Record expression contains both named and positional arguments
    #[error("Record expression contains both named and positional arguments")]
    MixedRecordArguments,

    /// Duplicate named argument
    #[error("Duplicate named argument: {0}")]
    DuplicateNamedArgument(Identifier),

    /// Positional argument after named argument
    #[error("Positional argument after named argument")]
    PositionalArgumentAfterNamed,

    /// Empty record expression
    #[error("Empty record expression")]
    EmptyRecordExpression,

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

    /// Loading of a source file failed
    #[error("Loading of source file {0:?} failed")]
    LoadSource(std::path::PathBuf),

    /// Grammar rule error
    #[error("Grammar rule error")]
    GrammarRuleError(String),
}

/// Result with parse error
pub type ParseResult<T> = Result<T, ParseError>;
