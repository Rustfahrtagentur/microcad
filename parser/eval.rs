use std::collections::HashMap;
use std::vec;

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
    pub fn new(node: SyntaxNode) -> Self {
        Self {
            stack: vec![SymbolTable::default()],
        }
    }

    pub fn push(&mut self) {
        self.stack.push(SymbolTable::default());
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn insert(&mut self, name: String, value: Value) {
        self.stack.last_mut().unwrap().insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        for table in self.stack.iter().rev() {
            if let Some(value) = table.get(name) {
                return Some(value);
            }
        }
        None
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
