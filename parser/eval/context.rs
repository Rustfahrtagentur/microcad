use super::{EvalError, Symbol, SymbolTable, Symbols};
use crate::language::identifier::*;
use microcad_render::tree;

/// @brief Context for evaluation
/// @details The context is used to store the current state of the evaluation.
/// A context is essentially a stack of symbol tables
#[derive(Debug)]
pub struct Context {
    stack: Vec<SymbolTable>,
    //    type_registry: HashMap<String, SyntaxNode>,
    current_node: tree::Node,
}

impl Context {
    pub fn push(&mut self) {
        self.stack.push(SymbolTable::default());
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn get_symbols_by_qualified_name(
        &self,
        name: &QualifiedName,
    ) -> Result<Vec<Symbol>, EvalError> {
        name.get_symbols(self)
    }

    pub fn current_node(&self) -> tree::Node {
        self.current_node.clone()
    }

    pub fn set_current_node(&mut self, node: tree::Node) {
        self.current_node = node;
    }

    /// Append a node to the current node and return the new node
    pub fn append_node(&mut self, node: tree::Node) -> tree::Node {
        self.current_node.append(node.clone());
        node.clone()
    }
}

impl Symbols for Context {
    fn find_symbols(&self, name: &Identifier) -> Vec<&Symbol> {
        self.stack
            .iter()
            .rev()
            .flat_map(|table| table.find_symbols(name))
            .collect()
    }

    fn add_symbol(&mut self, symbol: Symbol) -> &mut Self {
        self.stack.last_mut().unwrap().add_symbol(symbol);
        self
    }

    fn copy_symbols<T: Symbols>(&self, into: &mut T) {
        self.stack.last().unwrap().iter().for_each(|symbol| {
            into.add_symbol(symbol.clone());
        });
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            stack: vec![SymbolTable::default()],
            current_node: tree::root(),
        }
    }
}

// @todo Move this test elsewhere
#[test]
fn context_basic() {
    use crate::{
        eval::Eval,
        language::{assignment::Assignment, value::Value},
        parser::*,
    };

    let mut context = Context::default();

    context.add_value("a".into(), Value::Integer(1));
    context.add_value("b".into(), Value::Integer(2));

    assert_eq!(context.find_symbols(&"a".into())[0].name(), "a");
    assert_eq!(context.find_symbols(&"b".into())[0].name(), "b");

    let c = Parser::parse_rule_or_panic::<Assignment>(Rule::assignment, "c = a + b");

    c.eval(&mut context).unwrap();
}
