use crate::parse::*;
use thiserror::Error;

/// Parsing errors
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Expected identifier")]
    ExpectedIdentifier,

    #[error("Invalid use statement")]
    InvalidUseStatement,

    #[error("Error parsing floating point literal: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),

    #[error("Error parsing integer literal: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Error parsing color literal: {0}")]
    ParseColorError(String),

    #[error("Unknown unit: {0}")]
    UnknownUnit(String),

    #[error("Unexpected token")]
    UnexpectedToken,

    #[error("Tuple expression contains both named and positional arguments")]
    MixedTupleArguments,

    #[error("Duplicate named argument: {0}")]
    DuplicateNamedArgument(Identifier),

    #[error("Positional argument after named argument")]
    PositionalArgumentAfterNamed,

    #[error("Empty tuple expression")]
    EmptyTupleExpression,

    #[error("Missing type or value for definition parameter: {0}")]
    ParameterMissingTypeOrValue(Identifier),

    #[error("Duplicate parameter: {0}")]
    DuplicateParameter(Identifier),

    #[error("Duplicate argument: {0}")]
    DuplicateCallArgument(Identifier),

    #[error("Invalid map key type: {0}")]
    InvalidMapKeyType(String),

    #[error("Duplicated field name in map: {0}")]
    DuplicatedMapField(Identifier),

    #[error("Duplicate identifier: {0}")]
    DuplicateIdentifier(Identifier),
}

pub type ParseResult<T> = Result<T, ParseError>;
