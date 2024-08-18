use crate::language::{identifier::*, lang_type::*, value::*};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OperatorError {
    #[error("Invalid operator: {0}")]
    InvalidOperator(String),

    #[error("Incompatible types {0} and {1} for addition")]
    AddIncompatibleTypes(Type, Type),

    #[error("Incompatible types {0} and {1} for subtraction")]
    SubIncompatibleTypes(Type, Type),

    #[error("Incompatible types {0} and {1} for multiplication")]
    MulIncompatibleTypes(Type, Type),

    #[error("Incompatible types {0} and {1} for division")]
    DivIncompatibleTypes(Type, Type),
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid type: {0}")]
    InvalidType(Type),

    #[error("Operator error: {0}")]
    OperatorError(#[from] OperatorError),

    #[error("List index out of bounds: {index} >= {len}")]
    ListIndexOutOfBounds { index: usize, len: usize },

    #[error("Type mismatch: expected {expected}, got {found}")]
    TypeMismatch { expected: Type, found: Type },

    #[error("Cannot evaluate to type: {0}")]
    EvaluateToTypeError(Type),

    #[error("Value error {0}")]
    ValueError(#[from] ValueError),

    #[error("Unknown qualified name: {0}")]
    UnknownQualifiedName(QualifiedName),

    #[error("Unknown method: {0}")]
    UnknownMethod(Identifier),

    #[error("Elements of list have different types")]
    ListElementsDifferentTypes,

    #[error("Unknown error")]
    Unknown,

    #[error("Function call missing argument: {0}")]
    FunctionCallMissingArgument(Identifier),

    #[error("Function must return a value")]
    FunctionCallMissingReturn,

    #[error("Symbol not found: {0}")]
    SymbolNotFound(QualifiedName),

    #[error("Argument count mismatch: expected {expected}, got {found}")]
    ArgumentCountMismatch { expected: usize, found: usize },

    #[error("Invalid argument type: {0}")]
    InvalidArgumentType(Type),

    #[error("Expected module: {0}")]
    ExpectedModule(QualifiedName),

    #[error("Cannot nest function call")]
    CannotNestFunctionCall,

    #[error("Missing arguments: {0}")]
    MissingArguments(IdentifierList),

    #[error("Parameter type mismatch: {0} expected {1}, got {2}")]
    ParameterTypeMismatch(Identifier, Type, Type),

    #[error("Parameter missing type or value: {0}")]
    ParameterMissingTypeOrValue(Identifier),

    #[error("Unexpected argument: {0}")]
    UnexpectedArgument(Identifier),
}
