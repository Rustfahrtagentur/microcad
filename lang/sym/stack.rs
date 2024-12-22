// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Call stack for evaluation

use crate::{parse::*, src_ref::*, sym::*};

/// Stack frame in the context
///
/// It is used to store the current state of the evaluation.
/// A stack frame defines which kind of symbol we are currently evaluating.
#[derive(Debug, Clone, Default)]
pub struct StackFrame {
    source: std::rc::Rc<Symbol>,
    symbol_table: SymbolTable,
}

impl StackFrame {
    /// Create a new stack frame for a function
    pub fn function<C>(_: &mut C, function: std::rc::Rc<FunctionDefinition>) -> Self {
        Self {
            source: std::rc::Rc::new(Symbol::Function(function.clone())),
            symbol_table: SymbolTable::default(),
        }
    }

    /// Create a new stack frame for a module
    pub fn module(
        context: &mut impl Context,
        module: std::rc::Rc<crate::parse::ModuleDefinition>,
    ) -> SymResult<Self> {
        Ok(Self {
            source: std::rc::Rc::new(Symbol::Module(module.clone())),
            symbol_table: context.top()?.symbol_table.clone(),
        })
    }

    /// Create a new stack frame for a namespace
    pub fn namespace(
        context: &mut impl Context,
        namespace: std::rc::Rc<crate::parse::NamespaceDefinition>,
    ) -> SymResult<Self> {
        Ok(Self {
            source: std::rc::Rc::new(Symbol::Namespace(namespace.clone())),
            symbol_table: context.top()?.symbol_table.clone(),
        })
    }

    /// copy symbols from another symbol table
    pub fn copy<T: Symbols>(&self, into: &mut T) -> SymResult<()> {
        self.symbol_table.copy(into)
    }
}

impl Symbols for StackFrame {
    fn fetch(&self, id: &Id) -> Option<std::rc::Rc<Symbol>> {
        self.symbol_table.fetch(id)
    }

    fn add(&mut self, symbol: Symbol) -> &mut Self {
        self.symbol_table.add(symbol);
        self
    }

    fn add_alias(&mut self, symbol: Symbol, alias: Id) -> &mut Self {
        self.symbol_table.add_alias(symbol, alias);
        self
    }

    fn copy<T: Symbols>(&self, into: &mut T) -> SymResult<()> {
        self.symbol_table.copy(into)
    }
}

impl std::fmt::Display for StackFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.source.id().unwrap_or("root".into()))
    }
}

/// Call stack
///
/// By default, the stack contains a single stack frame with an empty symbol table.
#[derive(Debug, Clone)]
pub struct Stack(Vec<StackFrame>);

impl Stack {
    /// Push a new stack frame to the stack
    pub fn push(&mut self, stack_frame: StackFrame) {
        self.0.push(stack_frame);
    }

    /// Pop the top stack frame from the stack
    pub fn pop(&mut self) {
        self.0.pop();
    }

    /// Get the top stack frame
    pub fn top(&self) -> SymResult<&StackFrame> {
        if let Some(last) = self.0.last() {
            Ok(last)
        } else {
            Err(SymError::StackUnderflow)
        }
    }

    /// Get a mutual reference to the top stack frame
    pub fn top_mut(&mut self) -> &mut StackFrame {
        self.0.last_mut().expect("Empty stack")
    }

    /// Pretty print the stack
    pub fn pretty_print(
        &self,
        w: &mut dyn std::io::Write,
        source_file_by_hash: &impl crate::parse::GetSourceFileByHash,
    ) -> std::io::Result<()> {
        for (idx, stack_frame) in self.0.iter().rev().enumerate() {
            writeln!(w, "#{idx}\t{stack_frame}")?;
            // Print source location
            match stack_frame.source.src_ref() {
                SrcRef(None) => {}
                src_ref => {
                    let source_file = source_file_by_hash
                        .get_source_file_by_hash(src_ref.source_hash())
                        .expect("Source file not found");
                    writeln!(
                        w,
                        "\tat {filename}:{at}",
                        filename = source_file.filename_as_str(),
                        at = src_ref.at().expect("No source location"),
                    )?;
                }
            }
        }
        Ok(())
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self(vec![StackFrame::default()])
    }
}

impl std::ops::Deref for Stack {
    type Target = Vec<StackFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
