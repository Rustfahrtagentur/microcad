use crate::r#type::*;
use microcad_core::Id;
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
pub enum EvalError {
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
    ValueError(#[from] crate::eval::value::ValueError),

    #[error("Unknown qualified name: {0}")]
    UnknownQualifiedName(Id),

    #[error("Unknown method: {0}")]
    UnknownMethod(Id),

    #[error("Elements of list have different types")]
    ListElementsDifferentTypes,

    #[error("Unknown error")]
    Unknown,

    #[error("Function call missing argument: {0}")]
    FunctionCallMissingArgument(Id),

    #[error("Function must return a value")]
    FunctionCallMissingReturn,

    #[error("Symbol not found: {0}")]
    SymbolNotFound(Id),

    #[error("Argument count mismatch: expected {expected}, got {found}")]
    ArgumentCountMismatch { expected: usize, found: usize },

    #[error("Invalid argument type: {0}")]
    InvalidArgumentType(Type),

    #[error("Expected module: {0}")]
    ExpectedModule(Id),

    #[error("Cannot nest function call")]
    CannotNestFunctionCall,

    #[error("Missing arguments: {0:?}")]
    MissingArguments(Vec<Id>),

    #[error("Parameter type mismatch: {0} expected {1}, got {2}")]
    ParameterTypeMismatch(Id, Type, Type),

    #[error("Parameter missing type or value: {0}")]
    ParameterMissingTypeOrValue(Id),

    #[error("Unexpected argument: {0}")]
    UnexpectedArgument(Id),

    #[error("duplicate call argument: {0}")]
    DuplicateCallArgument(Id),

    #[error("Duplicate parameter: {0}")]
    DuplicateParameter(Id),

    #[error("Assertion failed: {0}")]
    AssertionFailed(String),
}
