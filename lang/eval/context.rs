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
    /// Source File
    SourceFile(std::rc::Rc<SourceFile>, SymbolTable),
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
            Self::SourceFile(_, table) => table,
            Self::Namespace(table) => table,
            Self::ModuleCall(table, _) => table,
            Self::FunctionCall(table) => table,
        }
    }

    /// Get a mutual reference to the symbol table
    pub fn symbol_table_mut(&mut self) -> &mut SymbolTable {
        match self {
            Self::SourceFile(_, table) => table,
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
        self.symbol_table().fetch(id)
    }

    fn add(&mut self, symbol: Symbol) -> &mut Self {
        self.symbol_table_mut().add(symbol);
        self
    }

    fn add_alias(&mut self, symbol: Symbol, alias: Id) -> &mut Self {
        self.symbol_table_mut().add_alias(symbol, alias);
        self
    }

    fn copy<T: Symbols>(&self, into: &mut T) {
        self.symbol_table().copy(into);
    }
}

/// Context for evaluation
///
/// The context is used to store the current state of the evaluation.
/// A context is essentially a stack of symbol tables
pub struct Context {
    /// Stack of symbol tables
    stack: Vec<StackFrame>,

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
