use std::collections::HashMap;
use std::rc::Rc;
use std::vec;
use thiserror::Error;

use crate::function::FunctionDefinition;
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
}

#[derive(Clone)]
pub enum Symbol {
    Value(Identifier, Value),
    Function(FunctionDefinition),
}

impl Symbol {
    pub fn name(&self) -> &str {
        match self {
            Self::Value(decl, _) => decl.into(),
            Self::Function(decl) => (&decl.name).into(),
        }
    }
}

/// @brief Symbol table
/// @details A symbol table is a mapping of symbol
#[derive(Default, Clone)]
pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn add(&mut self, symbol: Symbol) {
        self.symbols.insert(symbol.name().to_string(), symbol);
    }

    pub fn get(&self, name: &str) -> Option<&Symbol> {
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

    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.stack.last_mut().unwrap().add(symbol);
    }

    pub fn get_symbol(&self, name: impl Into<String>) -> Option<&Symbol> {
        let name = name.into();
        for table in self.stack.iter().rev() {
            if let Some(symbol) = table.get(&name) {
                return Some(symbol);
            }
        }
        None
    }

    pub fn resolve(&self, name: &QualifiedName) -> Result<&Symbol, Error> {
        // TODO: handle qualified names
        // We only handle the last piece of the qualified name
        let last = name.last();
        let symbol = self.get_symbol(last.clone());
        if let Some(symbol) = symbol {
            Ok(symbol)
        } else {
            Err(Error::UnknownQualifiedName(name.clone()))
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            stack: vec![SymbolTable::default()],
        }
    }
}

pub trait Eval {
    type Output;

    /// Evaluate the type into an expression
    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error>;
}

#[cfg(test)]
mod tests {
    use crate::parser::{Parser, Rule};

    #[test]
    fn context_basic() {
        use crate::eval::*;
        let mut context = Context::default();

        context.add_symbol(Symbol::Value("a".into(), Value::Integer(1)));
        context.add_symbol(Symbol::Value("b".into(), Value::Integer(2)));

        assert_eq!(context.get_symbol("a").unwrap().name(), "a");
        assert_eq!(context.get_symbol("b").unwrap().name(), "b");

        let _c = Parser::parse_rule_or_panic::<crate::function::Assignment>(
            Rule::assignment,
            "c = a + b",
        );

        //c.eval(Some(&context)).unwrap();
    }
}
