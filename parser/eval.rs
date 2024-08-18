use std::ops::Deref;

use crate::language::{function::*, identifier::*, lang_type::*, module::*, value::*};
use microcad_render::tree::Node;
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

#[derive(Clone, Debug)]
pub enum Symbol {
    Value(Identifier, Value),
    Function(std::rc::Rc<FunctionDefinition>),
    ModuleDefinition(std::rc::Rc<ModuleDefinition>),
    BuiltinFunction(BuiltinFunction),
    BuiltinModule(BuiltinModule),
}

impl Symbol {
    pub fn name(&self) -> &Identifier {
        match self {
            Self::Value(v, _) => v,
            Self::Function(f) => &f.name,
            Self::ModuleDefinition(m) => &m.name,
            Self::BuiltinFunction(f) => &f.name,
            Self::BuiltinModule(m) => &m.name,
        }
    }

    pub fn get_symbols(&self, name: &Identifier) -> Vec<&Symbol> {
        match self {
            Self::ModuleDefinition(module) => module.get_symbols_by_name(name),
            _ => Vec::new(),
        }
    }
}

/// @brief Symbol table
/// @details A symbol table is a mapping of symbol
#[derive(Clone, Debug, Default)]
pub struct SymbolTable {
    symbols: Vec<Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),
        }
    }

    pub fn add(&mut self, symbol: Symbol) {
        self.symbols.push(symbol);
    }

    pub fn get(&self, name: &Identifier) -> Vec<&Symbol> {
        let mut symbols = Vec::new();
        for symbol in self.symbols.iter() {
            if symbol.name() == name {
                symbols.push(symbol);
            }
        }
        symbols
    }
}

impl Deref for SymbolTable {
    type Target = Vec<Symbol>;

    fn deref(&self) -> &Self::Target {
        &self.symbols
    }
}

/// @brief Context for evaluation
/// @details The context is used to store the current state of the evaluation.
/// A context is essentially a stack of symbol tables
#[derive(Debug)]
pub struct Context {
    stack: Vec<SymbolTable>,
    //    type_registry: HashMap<String, SyntaxNode>,
    current_node: microcad_render::tree::Node,
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

    pub fn get_symbols(&self, name: &Identifier) -> Vec<&Symbol> {
        let mut symbols = Vec::new();
        for table in self.stack.iter().rev() {
            symbols.extend(table.get(name));
        }
        symbols
    }

    pub fn get_symbols_by_qualified_name(
        &self,
        name: &QualifiedName,
    ) -> Result<Vec<Symbol>, Error> {
        name.get_symbols(self)
    }

    pub fn current_node(&self) -> Node {
        self.current_node.clone()
    }

    pub fn set_current_node(&mut self, node: Node) {
        self.current_node = node;
    }

    /// Append a node to the current node and return the new node
    pub fn append_node(&mut self, node: Node) -> Node {
        self.current_node.append(node.clone());
        node.clone()
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            stack: vec![SymbolTable::default()],
            current_node: microcad_render::tree::root(),
        }
    }
}

pub trait Eval {
    type Output;

    /// Evaluate the type into an expression
    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error>;
}

#[test]
fn context_basic() {
    use crate::parser::*;

    let mut context = Context::default();

    context.add_symbol(Symbol::Value("a".into(), Value::Integer(1)));
    context.add_symbol(Symbol::Value("b".into(), Value::Integer(2)));

    assert_eq!(context.get_symbols(&"a".into())[0].name(), "a");
    assert_eq!(context.get_symbols(&"b".into())[0].name(), "b");

    let _c = Parser::parse_rule_or_panic::<Assignment>(Rule::assignment, "c = a + b");

    //c.eval(Some(&context)).unwrap();
}
