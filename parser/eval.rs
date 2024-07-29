use std::collections::HashMap;

use crate::langtype::Type;
use crate::syntax_tree::SyntaxNode;
use crate::value::{Value, ValueError};

#[derive(Debug)]
pub enum Error {
    InvalidOperation,
    InvalidFormatString,
    InvalidType,
    ListIndexOutOfBounds { index: usize, len: usize },
    TypeMismatch,
    EvaluateToStringError,
    ValueError(ValueError),
}

impl From<ValueError> for Error {
    fn from(value_error: ValueError) -> Self {
        Error::ValueError(value_error)
    }
}

// Context for evaluation
pub struct Context {
    node: SyntaxNode,
    symbols: HashMap<String, SyntaxNode>,
    //    type_registry: HashMap<String, SyntaxNode>,
}

impl Context {
    pub fn new(node: SyntaxNode) -> Self {
        Self {
            node,
            symbols: HashMap::new(),
        }
    }
}

pub trait Eval: Sized {
    /// Evaluate the type into an expression
    fn eval(self, context: Option<&Context>) -> Result<Value, Error>;

    /// The expected destination type after evaluation
    fn eval_type(&self, context: Option<&Context>) -> Result<Type, crate::eval::Error>;
}

pub trait EvalTo<T> {
    fn eval_to(self, context: Option<&Context>) -> Result<T, Error>;
}
