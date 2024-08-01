use std::collections::HashMap;
use std::vec;
use thiserror::Error;

use crate::identifier::{Identifier, QualifiedName};
use crate::lang_type::Type;
use crate::value::{Value, ValueError};

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
    #[error("Type mismatch: expected {0}, got {1}")]
    TypeMismatch(Type, Type),
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
}

/// @brief Symbol table
/// @details A symbol table is a mapping of symbol names to their corresponding syntax nodes.
#[derive(Default, Clone)]
pub struct SymbolTable {
    symbols: HashMap<String, Value>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, value: Value) {
        self.symbols.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.symbols.get(name)
    }
}

/// @brief Context for evaluation
/// @details The context is used to store the current state of the evaluation.
/// A context is essentially a stack of symbol tables
pub struct Context {
    stack: Vec<SymbolTable>,
    //    type_registry: HashMap<String, SyntaxNode>,
}

impl Context {
    pub fn push(&mut self) {
        self.stack.push(SymbolTable::default());
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn insert(&mut self, name: impl Into<String>, value: Value) {
        self.stack.last_mut().unwrap().insert(name.into(), value);
    }

    pub fn get(&self, name: impl Into<String>) -> Option<&Value> {
        let name = name.into();
        for table in self.stack.iter().rev() {
            if let Some(value) = table.get(&name) {
                return Some(value);
            }
        }
        None
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            stack: vec![SymbolTable::default()],
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
