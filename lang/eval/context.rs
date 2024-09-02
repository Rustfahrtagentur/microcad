use super::{Eval, EvalError, Symbol, SymbolTable, Symbols};
use crate::{diagnostics::{Diagnostic, AddDiagnostic, Diagnostics}, parse::{identifier::*, SourceFile}};
use microcad_core::Id;
use microcad_render::tree;

/// Context for evaluation
///
/// The context is used to store the current state of the evaluation.
/// A context is essentially a stack of symbol tables
#[derive(Debug)]
pub struct Context {
    /// Stack of symbol tables
    stack: Vec<SymbolTable>,

    /// Current node in the tree where the evaluation is happening
    current_node: tree::Node,

    /// Source files loaded in the context
    source_files: std::collections::HashMap<String, std::rc::Rc<SourceFile>>,

    /// Source file diagnostics
    diagnostics: Diagnostics,
}

impl Context {
    pub fn from_source_file(source_file: SourceFile) -> Self {
        let mut context = Self::default();
        context.add_source_file(source_file);
    
        context
    }

    /// Evaluate the context with the current source file
    pub fn eval(&mut self) -> super::Result<tree::Node> {
        let node = self.current_source_file().eval(self)?;

        self.info(crate::src_ref::SrcRef(None), "Evaluation complete".into());
        Ok(node)
    }

    pub fn current_source_file(&self) -> std::rc::Rc<SourceFile> {
        self.diagnostics.current_source_file().clone()
    }

    /// Read-only access to the diagnostics
    pub fn diagnostics(&self) -> &Diagnostics {
        &self.diagnostics
    }

    /// Add a new source file to the context and set it as the current source file
    pub fn add_source_file(&mut self, source_file: SourceFile) {
        let new_source_file = std::rc::Rc::new(source_file);
        self.source_files.insert(new_source_file.filename().to_string(), new_source_file.clone());
    
        self.diagnostics.push(new_source_file.clone());
    }

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

impl AddDiagnostic for Context {
    fn add_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.add_diagnostic(diagnostic);
    }
}

impl Symbols for Context {
    fn find_symbols(&self, id: &Id) -> Vec<&Symbol> {
        self.stack
            .iter()
            .rev()
            .flat_map(|table| table.find_symbols(id))
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
            source_files: std::collections::HashMap::new(),
            diagnostics: Diagnostics::default(),
        }
    }
}

// @todo Move this test elsewhere
#[test]
fn context_basic() {
    use crate::{eval::*, parse::*, parser::*};

    let mut context = Context::default();

    context.add_value("a".into(), Value::Integer(1));
    context.add_value("b".into(), Value::Integer(2));

    assert_eq!(context.find_symbols(&"a".into())[0].id().unwrap(), "a");
    assert_eq!(context.find_symbols(&"b".into())[0].id().unwrap(), "b");

    let c = Parser::parse_rule_or_panic::<Assignment>(Rule::assignment, "c = a + b");

    c.eval(&mut context).unwrap();
}
