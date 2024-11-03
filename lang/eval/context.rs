// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Eval, EvalError, Symbol, SymbolTable, Symbols};
use crate::{diag::*, objecttree::*, parse::*, source_file_cache::*};

use microcad_core::Id;

/// Stack frame in the context
///
/// It is used to store the current state of the evaluation.
/// A stack frame defines which kind of symbol we are currently evaluating.
#[derive(Debug, Clone)]
pub enum StackFrame {
    /// Initial state
    Namespace(SymbolTable),
    /// Currently evaluating a module definition
    ModuleCall(SymbolTable, Option<ObjectNode>),
    /// Currently evaluating a function definition
    FunctionCall(SymbolTable),
}

impl StackFrame {
    /// Get the symbol table of the stack frame
    pub fn symbol_table(&self) -> &SymbolTable {
        match self {
            Self::Namespace(table) => table,
            Self::ModuleCall(table, _) => table,
            Self::FunctionCall(table) => table,
        }
    }
}

impl Default for StackFrame {
    fn default() -> Self {
        Self::Namespace(SymbolTable::default())
    }
}

impl Symbols for StackFrame {
    fn fetch(&self, id: &Id) -> Option<std::rc::Rc<Symbol>> {
        match self {
            Self::Namespace(table) => table.fetch(id),
            Self::ModuleCall(table, _) => table.fetch(id),
            Self::FunctionCall(table) => table.fetch(id),
        }
    }

    fn add(&mut self, symbol: Symbol) -> &mut Self {
        match self {
            Self::Namespace(table) => table.add(symbol),
            Self::ModuleCall(table, _) => table.add(symbol),
            Self::FunctionCall(table) => table.add(symbol),
        };
        self
    }

    fn add_alias(&mut self, symbol: Symbol, alias: Id) -> &mut Self {
        match self {
            Self::Namespace(table) => table.add_alias(symbol, alias),
            Self::ModuleCall(table, _) => table.add_alias(symbol, alias),
            Self::FunctionCall(table) => table.add_alias(symbol, alias),
        };
        self
    }

    fn copy<T: Symbols>(&self, into: &mut T) {
        match self {
            Self::Namespace(table) => table.copy(into),
            Self::ModuleCall(table, _) => table.copy(into),
            Self::FunctionCall(table) => table.copy(into),
        }
    }
}

/// Context for evaluation
///
/// The context is used to store the current state of the evaluation.
/// A context is essentially a stack of symbol tables
pub struct Context {
    /// Stack of symbol tables
    stack: Vec<StackFrame>,

    /// Current node in the tree where the evaluation is happening
    current_node: ObjectNode,

    /// Current source file being evaluated
    current_source_file: Option<std::rc::Rc<SourceFile>>,

    /// Source file cache containing all source files loaded in the context
    source_files: SourceFileCache,

    /// Source file diagnostics
    diag_handler: DiagHandler,
}

impl Context {
    /// Create a new context from a source file
    pub fn from_source_file(source_file: SourceFile) -> Self {
        let rc_source_file = std::rc::Rc::new(source_file);

        let mut ctx = Self {
            stack: vec![StackFrame::default()],
            current_node: group(),
            current_source_file: Some(rc_source_file.clone()),
            ..Default::default()
        };

        ctx.source_files.add(rc_source_file);
        ctx
    }

    /// Evaluate the context with the current source file
    pub fn eval(&mut self) -> super::Result<ObjectNode> {
        let node = self.current_source_file().unwrap().eval(self)?;
        self.info(crate::src_ref::SrcRef(None), "Evaluation complete".into());
        Ok(node)
    }

    /// Return the current source file
    ///
    /// Note: This should not be an optional value, as the context is always created with a source file
    pub fn current_source_file(&self) -> Option<std::rc::Rc<SourceFile>> {
        self.current_source_file.clone()
    }

    /// Push a new symbol table to the stack (enter a new scope)
    fn push(&mut self, stack_frame: StackFrame) -> &mut StackFrame {
        self.stack.push(stack_frame);
        self.stack.last_mut().unwrap()
    }

    /// Pop the top symbol table from the stack (exit the current scope)
    fn pop(&mut self) {
        self.stack.pop();
    }

    /// The top symbol table in the stack
    pub fn top(&self) -> &StackFrame {
        self.stack.last().unwrap()
    }

    /// The top symbol table in the stack (mutable)
    pub fn top_mut(&mut self) -> &mut StackFrame {
        self.stack.last_mut().unwrap()
    }

    /// Create a new symbol table and push it to the stack
    pub fn scope(
        &mut self,
        stack_frame: StackFrame,
        f: impl FnOnce(&mut Self) -> crate::eval::Result<()>,
    ) -> crate::eval::Result<()> {
        self.push(stack_frame);
        f(self)?;
        self.pop();
        Ok(())
    }

    /// Set new_node as current node, call function and set old node
    pub fn descend_node<F>(&mut self, new_node: ObjectNode, f: F) -> crate::eval::Result<ObjectNode>
    where
        F: FnOnce(&mut Self) -> crate::eval::Result<ObjectNode>,
    {
        let old_node = self.current_node.clone();
        self.set_current_node(new_node.clone());
        f(self)?;
        self.set_current_node(old_node);
        Ok(new_node)
    }

    /// Read-only access to diagnostic handler
    pub fn diag(&self) -> &DiagHandler {
        &self.diag_handler
    }

    /// Fetch symbols by qualified name
    pub fn fetch_symbols_by_qualified_name(
        &mut self,
        name: &QualifiedName,
    ) -> Result<Vec<Symbol>, EvalError> {
        name.fetch_symbols(self)
    }

    /// Get current evaluation node
    pub fn current_node(&self) -> ObjectNode {
        self.current_node.clone()
    }

    /// Set current evaluation node
    pub fn set_current_node(&mut self, node: ObjectNode) {
        self.current_node = node;
    }

    /// Append a node to the current node and return the new node
    pub fn append_node(&mut self, node: ObjectNode) -> ObjectNode {
        self.current_node.append(node.clone());
        node.clone()
    }

    /// Add source file to Context
    pub fn add_source_file(&mut self, source_file: SourceFile) {
        self.source_files.add(std::rc::Rc::new(source_file))
    }
}

impl PushDiag for Context {
    fn push_diag(&mut self, diag: Diag) -> crate::eval::Result<()> {
        self.diag_handler.push_diag(diag)
    }
}

impl Symbols for Context {
    fn fetch(&self, id: &Id) -> Option<std::rc::Rc<Symbol>> {
        self.stack
            .iter()
            .rev()
            .flat_map(|stack_frame| stack_frame.fetch(id))
            .next()
    }

    fn add(&mut self, symbol: Symbol) -> &mut Self {
        self.stack.last_mut().unwrap().add(symbol);
        self
    }

    fn add_alias(&mut self, symbol: Symbol, alias: Id) -> &mut Self {
        self.stack.last_mut().unwrap().add_alias(symbol, alias);
        self
    }

    fn copy<T: Symbols>(&self, into: &mut T) {
        self.top().symbol_table().iter().for_each(|(_, symbol)| {
            into.add(symbol.as_ref().clone());
        });
    }
}

impl GetSourceFileByHash for Context {
    fn get_source_file_by_hash(&self, hash: u64) -> Option<&SourceFile> {
        self.source_files.get_source_file_by_hash(hash)
    }
}

/// Default implementation for the context
/// TODO: Remove this, it's just for testing
impl Default for Context {
    fn default() -> Self {
        Self {
            stack: vec![StackFrame::default()],
            current_node: group(),
            current_source_file: None,
            source_files: SourceFileCache::default(),
            diag_handler: DiagHandler::default(),
        }
    }
}

// @todo Move this test elsewhere
#[test]
fn context_basic() {
    use crate::{eval::*, parse::*, parser::*, src_ref::*};

    let mut context = Context::default();

    context.add_value("a".into(), Value::Integer(Refer::none(1)));
    context.add_value("b".into(), Value::Integer(Refer::none(2)));

    assert_eq!(context.fetch(&"a".into()).unwrap().id().unwrap(), "a");
    assert_eq!(context.fetch(&"b".into()).unwrap().id().unwrap(), "b");

    let c = Parser::parse_rule::<Assignment>(Rule::assignment, "c = a + b", 0).unwrap();

    c.eval(&mut context).unwrap();
}
