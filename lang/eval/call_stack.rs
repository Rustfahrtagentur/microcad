// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Call stack

use crate::{eval::*, resolve::*, src_ref::*, syntax::*};

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
            src_ref: src_ref.src_ref(),
        }
    }

    /// Pretty print single call stack frame
    fn pretty_print(
        &self,
        f: &mut dyn std::fmt::Write,
        source_by_hash: &impl GetSourceByHash,
        idx: usize,
    ) -> std::fmt::Result {
        writeln!(f, "{:>4}: {name}", idx, name = self.symbol_node.full_name())?;

        if let Some(line_col) = self.src_ref.at() {
            let source_file = source_by_hash.get_by_hash(self.src_ref.source_hash());
            writeln!(
                f,
                "            at {filename}:{line_col}",
                filename = source_file
                    .as_ref()
                    .map(|sf| sf.filename_as_str())
                    .unwrap_or(SourceFile::NO_FILE),
            )?;
        }

        Ok(())
    }
}

/// Call stack
#[derive(Default, Debug, Clone)]
pub struct CallStack(Vec<CallStackFrame>);

/// Diagnosis trait gives access about collected errors
pub trait CallTrace {
    /// Pretty print all calls
    fn fmt_calls(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result;

    /// Pretty write all calls into a file
    fn write_calls(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        write!(w, "{}", self.calls())
    }

    /// Get pretty printed calls as string
    fn calls(&self) -> String {
        let mut str = String::new();
        self.fmt_calls(&mut str).expect("displayable diagnosis");
        str
    }
    /// Push to stack
    fn push(&mut self, symbol_node: SymbolNode, args: ArgumentMap, src_ref: impl SrcReferrer);

    /// Pop from stack
    fn pop(&mut self);
}

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
    pub fn pretty_print(
        &self,
        f: &mut dyn std::fmt::Write,
        source_by_hash: &impl super::GetSourceByHash,
    ) -> std::fmt::Result {
        for (idx, call_stack_frame) in self.0.iter().enumerate() {
            call_stack_frame.pretty_print(f, source_by_hash, idx)?;
        }
        Ok(())
    }
}
