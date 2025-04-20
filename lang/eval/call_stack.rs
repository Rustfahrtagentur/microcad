// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Call stack

use crate::{resolve::{FullyQualify, SymbolNode}, src_ref::{SrcRef, SrcReferrer}};

use super::ArgumentMap;


/// Stack frame for a single call
/// 
/// Multiplicity calls are separate into single calls
#[derive(Debug, Clone)]
pub struct CallStackFrame {
    /// Symbol that was call
    symbol_node: SymbolNode,

    /// Call arguments
    args: ArgumentMap,

    /// Source code reference
    src_ref: SrcRef,
}

impl CallStackFrame {
    /// Construct a new stack frame
    pub fn new(symbol_node: SymbolNode, args: ArgumentMap, src_ref: impl SrcReferrer) -> Self {
        Self {
            symbol_node,
            args,
            src_ref: src_ref.src_ref()
        }
    }

    /// Pretty print single call stack frame
    fn pretty_print(&self, f: &mut std::fmt::Formatter<'_>, source_by_hash: &impl super::GetSourceByHash, idx: usize) -> std::fmt::Result {
        match self.symbol_node.full_name() {
            Some(name) => writeln!(f, "{:>4}: {name}", idx)?,
            None => writeln!(f, "{:>4}: {id}", idx, id = self.symbol_node.id())?,
        };

        if let Some(line_col) = self.src_ref.at() {
            let source_file = source_by_hash.get_by_hash(self.src_ref.source_hash());
            writeln!(f, "            at {filename}:{line_col}", 
                filename = source_file
                    .as_ref()
                    .map(|sf| sf.filename_as_str())
                    .unwrap_or(crate::syntax::SourceFile::NO_FILE), 
            )?;
        }

        Ok(())
    }
}


/// Call stack
#[derive(Default, Debug, Clone)]
pub struct CallStack(Vec<CallStackFrame>);

impl CallStack {
    /// Push to stack
    pub fn push(&mut self, symbol_node: SymbolNode, args: ArgumentMap, src_ref: impl SrcReferrer) {
        self.0.push(CallStackFrame::new(symbol_node, args, src_ref))
    }

    /// Pop from stack
    pub fn pop(&mut self) {
        self.0.pop();
    }

    /// Pretty print stack
    pub fn pretty_print(&self, f: &mut std::fmt::Formatter<'_>, source_by_hash: &impl super::GetSourceByHash) -> std::fmt::Result {
        for (idx, call_stack_frame) in self.0.iter().enumerate() {
            call_stack_frame.pretty_print(f, source_by_hash, idx)?;
        }
        Ok(())
    }
}

