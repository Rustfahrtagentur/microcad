// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Eval, EvalError, Symbol, SymbolTable, Symbols};
use crate::{diag::*, parse::*, source_file_cache::*};

use microcad_core::Id;
use microcad_render::tree;

/// Context for evaluation
///
/// The context is used to store the current state of the evaluation.
/// A context is essentially a stack of symbol tables
pub struct Context {
    /// Stack of symbol tables
    stack: Vec<SymbolTable>,

    /// Current node in the tree where the evaluation is happening
    current_node: tree::Node,

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
        Self {
            stack: vec![SymbolTable::default()],
            current_node: tree::group(),
            current_source_file: Some(std::rc::Rc::new(source_file)),
            ..Default::default()
        }
    }

    /// Evaluate the context with the current source file
    pub fn eval(&mut self) -> super::Result<tree::Node> {
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
    pub fn push(&mut self) {
        self.stack.push(SymbolTable::default());
    }

    /// Pop the top symbol table from the stack (exit the current scope)
    pub fn pop(&mut self) {
        self.stack.pop();
    }

    /// Set new_node as current node, call function and set old node
    pub fn descend_node<F>(
        &mut self,
        new_node: microcad_core::render::Node,
        f: F,
    ) -> crate::eval::Result<microcad_core::render::Node>
    where
        F: FnOnce(&mut Self) -> crate::eval::Result<microcad_core::render::Node>,
    {
        let old_node: rctree::Node<tree::NodeInner> = self.current_node.clone();
        self.set_current_node(new_node.clone());
        f(self)?;
        self.set_current_node(old_node);
        Ok(new_node)
    }

    /// Open a new scope and execute the given closure
    pub fn scope<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Self),
    {
        self.push();
        f(self);
        self.pop();
    }

    /// Read-only access to diagnostic handler
    pub fn diag(&self) -> &DiagHandler {
        &self.diag_handler
    }

    /// Fetch symbols by qualified name
    pub fn fetch_symbols_by_qualified_name(
        &self,
        name: &QualifiedName,
    ) -> Result<Vec<Symbol>, EvalError> {
        name.fetch_symbols(self)
    }

    /// Get current evaluation node
    pub fn current_node(&self) -> tree::Node {
        self.current_node.clone()
    }

    /// Set current evaluation node
    pub fn set_current_node(&mut self, node: tree::Node) {
        self.current_node = node;
    }

    /// Append a node to the current node and return the new node
    pub fn append_node(&mut self, node: tree::Node) -> tree::Node {
        self.current_node.append(node.clone());
        node.clone()
    }
}

impl PushDiag for Context {
    fn push_diag(&mut self, diag: Diag) -> crate::eval::Result<()> {
        self.diag_handler.push_diag(diag)
    }
}

impl Symbols for Context {
    fn fetch(&self, id: &Id) -> Vec<&Symbol> {
        self.stack
            .iter()
            .rev()
            .flat_map(|table| table.fetch(id))
            .collect()
    }

    fn add(&mut self, symbol: Symbol) -> &mut Self {
        self.stack.last_mut().unwrap().add(symbol);
        self
    }

    fn copy<T: Symbols>(&self, into: &mut T) {
        self.stack.last().unwrap().iter().for_each(|symbol| {
            into.add(symbol.clone());
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
            stack: vec![SymbolTable::default()],
            current_node: tree::group(),
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

    assert_eq!(context.fetch(&"a".into())[0].id().unwrap(), "a");
    assert_eq!(context.fetch(&"b".into())[0].id().unwrap(), "b");

    let c = Parser::parse_rule::<Assignment>(Rule::assignment, "c = a + b", 0).unwrap();

    c.eval(&mut context).unwrap();
}
