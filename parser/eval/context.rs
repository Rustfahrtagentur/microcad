use super::{Identifier, Symbol, SymbolTable, Symbols};
use microcad_render::tree;

/// Context for evaluation
///
/// The context is used to store the current state of the evaluation.
/// A context is essentially a stack of symbol tables
#[derive(Debug, Clone)]
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

    pub fn current_node(&self) -> &tree::Node {
        &self.current_node
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
    fn add_symbol(&mut self, symbol: Symbol) {
        self.stack.last_mut().unwrap().push(symbol);
    }

    fn find_symbols(&self, name: &Identifier) -> Vec<&Symbol> {
        self.stack
            .iter()
            .rev()
            .flat_map(|table| table.find_symbols(name))
            .collect()
    }
    fn copy_symbols<T: Symbols>(&self, symbols: &mut T) {
        self.stack.last().unwrap().copy_symbols(symbols)
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
