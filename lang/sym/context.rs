// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{diag::*, eval::*, objects::*, parse::*, source_file_cache::*, sym::*};

/// Context for evaluation
///
/// The context is used to store the current state of the evaluation.
/// A context is essentially a stack of symbol tables
#[derive(Default)]
pub struct Context {
    /// Stack of symbol tables
    stack: Stack,

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
            current_source_file: Some(rc_source_file.clone()),
            ..Default::default()
        };

        ctx.source_files.add(rc_source_file);
        ctx
    }

    /// Evaluate the context with the current source file
    pub fn eval(&mut self) -> EvalResult<ObjectNode> {
        let node = self
            .current_source_file()
            .expect("No current source file")
            .eval(self)?;
        Ok(node)
    }

    /// Return the current source file
    ///
    /// Note: This should not be an optional value, as the context is always created with a source file
    pub fn current_source_file(&self) -> Option<std::rc::Rc<SourceFile>> {
        self.current_source_file.clone()
    }

    /// Fetch symbols by qualified name
    pub fn fetch_symbols_by_qualified_name(
        &mut self,
        name: &QualifiedName,
    ) -> EvalResult<Vec<Symbol>> {
        name.fetch_symbols(self)
    }

    /// Add source file to Context
    pub fn add_source_file(&mut self, source_file: SourceFile) {
        self.source_files.add(std::rc::Rc::new(source_file))
    }

    /// Stack trace returns a copy of the current stack
    pub fn stack_trace(&self) -> Stack {
        self.stack.clone()
    }

    /// Error with stack trace
    pub fn error_with_stack_trace(
        &mut self,
        src_ref: impl crate::src_ref::SrcReferrer,
        error: impl std::error::Error + 'static,
    ) -> crate::eval::EvalResult<()> {
        self.error(src_ref, Box::new(error), Some(self.stack_trace()))
    }

    /// Push a new symbol table to the stack (enter a new scope)
    pub fn push(&mut self, stack_frame: StackFrame) {
        self.stack.push(stack_frame);
    }

    /// Pop the top symbol table from the stack (exit the current scope)
    pub fn pop(&mut self) {
        self.stack.pop();
    }

    /// The top symbol table in the stack
    ///
    /// This method guarantees that the stack is not empty
    pub fn top(&self) -> SymResult<&StackFrame> {
        self.stack.top()
    }

    /// The top symbol table in the stack (mutable)
    ///
    /// This method guarantees that the stack is not empty
    pub fn top_mut(&mut self) -> &mut StackFrame {
        self.stack.top_mut()
    }

    /// Create a new symbol table and push it to the stack
    pub fn scope<T>(
        &mut self,
        stack_frame: StackFrame,
        f: impl FnOnce(&mut Self) -> EvalResult<T>,
    ) -> EvalResult<T> {
        self.push(stack_frame);
        let t = f(self)?;
        self.pop();
        Ok(t)
    }

    /// Read-only access to diagnostic handler
    pub fn diag(&self) -> &DiagHandler {
        &self.diag_handler
    }
}

impl PushDiag for Context {
    fn push_diag(&mut self, diag: Diag) -> crate::eval::EvalResult<()> {
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
        self.top_mut().add(symbol);
        self
    }

    fn add_alias(&mut self, symbol: Symbol, alias: Id) -> &mut Self {
        self.top_mut().add_alias(symbol, alias);
        self
    }

    fn copy<T: Symbols>(&self, into: &mut T) -> SymResult<()> {
        self.top()?.copy(into)
    }
}

impl GetSourceFileByHash for Context {
    fn get_source_file_by_hash(&self, hash: u64) -> Option<&SourceFile> {
        self.source_files.get_source_file_by_hash(hash)
    }
}

// @todo Move this test elsewhere
#[test]
fn context_basic() {
    use crate::{eval::*, parse::*, parser::*, src_ref::*};

    let mut context = Context::default();

    context.add_value("a".into(), Value::Integer(Refer::none(1)));
    context.add_value("b".into(), Value::Integer(Refer::none(2)));

    assert_eq!(
        context
            .fetch(&"a".into())
            .expect("test error")
            .id()
            .expect("test error"),
        "a"
    );
    assert_eq!(
        context
            .fetch(&"b".into())
            .expect("test error")
            .id()
            .expect("test error"),
        "b"
    );

    let c = Parser::parse_rule::<Assignment>(Rule::assignment, "c = a + b", 0).expect("test error");

    c.eval(&mut context).expect("test error");
}
