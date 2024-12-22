// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Call stack for evaluation

use crate::{eval::symbols::*, eval::*, parse::FunctionDefinition};

/// Stack frame in the context
///
/// It is used to store the current state of the evaluation.
/// A stack frame defines which kind of symbol we are currently evaluating.
#[derive(Debug, Clone, Default)]
pub struct StackFrame {
    source: std::rc::Rc<Symbol>,
    symbol_table: SymbolTable,
    source_file: Option<std::rc::Rc<crate::parse::SourceFile>>,
}

impl StackFrame {
    pub fn function(context: &mut Context, function: std::rc::Rc<FunctionDefinition>) -> Self {
        Self {
            source: std::rc::Rc::new(Symbol::Function(function.clone())),
            symbol_table: SymbolTable::default(),
            source_file: context.current_source_file().clone(),
        }
    }

    pub fn module(
        context: &mut Context,
        module: std::rc::Rc<crate::parse::ModuleDefinition>,
    ) -> Self {
        Self {
            source: std::rc::Rc::new(Symbol::Module(module.clone())),
            symbol_table: context.top().symbol_table().clone(),
            source_file: context.current_source_file().clone(),
        }
    }

    pub fn namespace(
        context: &mut Context,
        namespace: std::rc::Rc<crate::parse::NamespaceDefinition>,
    ) -> Self {
        Self {
            source: std::rc::Rc::new(Symbol::Namespace(namespace.clone())),
            symbol_table: context.top().symbol_table().clone(),
            source_file: context.current_source_file().clone(),
        }
    }

    /// Get the symbol table of the stack frame
    pub fn symbol_table(&self) -> &SymbolTable {
        &self.symbol_table
    }

    /// Get a mutual reference to the symbol table
    pub fn symbol_table_mut(&mut self) -> &mut SymbolTable {
        &mut self.symbol_table
    }
}

impl Symbols for StackFrame {
    fn fetch(&self, id: &microcad_core::Id) -> Option<std::rc::Rc<Symbol>> {
        self.symbol_table().fetch(id)
    }

    fn add(&mut self, symbol: Symbol) -> &mut Self {
        self.symbol_table_mut().add(symbol);
        self
    }

    fn add_alias(&mut self, symbol: Symbol, alias: microcad_core::Id) -> &mut Self {
        self.symbol_table_mut().add_alias(symbol, alias);
        self
    }

    fn copy<T: Symbols>(&self, into: &mut T) {
        self.symbol_table().copy(into);
    }
}

impl std::fmt::Display for StackFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.source.id())
    }
}

pub struct Stack(Vec<StackFrame>);

impl Stack {
    pub fn push(&mut self, stack_frame: StackFrame) {
        self.0.push(stack_frame);
    }

    pub fn pop(&mut self) {
        self.0.pop();
    }

    pub fn top(&self) -> &StackFrame {
        self.0.last().expect("Empty stack")
    }

    pub fn top_mut(&mut self) -> &mut StackFrame {
        self.0.last_mut().expect("Empty stack")
    }

    pub fn pretty_print(
        &self,
        w: &mut dyn std::io::Write,
        source_file_by_hash: &impl crate::parse::GetSourceFileByHash,
    ) -> std::io::Result<()> {
        for (idx, stack_frame) in self.0.iter().rev().enumerate() {
            writeln!(w, "#{idx}\t{stack_frame}")?;
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
