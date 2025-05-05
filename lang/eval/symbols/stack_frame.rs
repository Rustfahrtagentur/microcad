// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

/// Stack frame for a single call.
#[derive(Debug, Clone)]
pub struct CallStackFrame {
    /// Symbol that was called.
    symbol: Symbol,

    /// Call arguments.
    args: CallArgumentList,

    /// Source code reference.
    src_ref: SrcRef,
}

impl CallStackFrame {
    /// Create new call stack frame.
    pub fn new(symbol: Symbol, args: CallArgumentList, src_ref: impl SrcReferrer) -> Self {
        Self {
            symbol,
            args,
            src_ref: src_ref.src_ref()
        }
    }

    /// Pretty print single call stack frame.
    pub fn pretty_print(
        &self,
        f: &mut dyn std::fmt::Write,
        source_by_hash: &impl GetSourceByHash,
        idx: usize,
    ) -> std::fmt::Result {
        writeln!(f, "{:>4}: {name}", idx, name = self.symbol.full_name())?;

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


/// Storage for *local variables* and *aliases* (for *use statements*).
///
/// A *stack frame* can have different types and some provide a storage for *local variables*
/// (like [`LocalFrame::Source`] and [`LocalFrame::Scope`]) and some do not, some are named
/// (like [`LocalFrame::Source`], [`LocalFrame::Namespace`] and [`LocalFrame::Module`])
/// and some do not.
pub enum StackFrame {
    /// Source file with locals.
    Source(Identifier, SymbolMap),
    /// Namespace scope without locals
    Namespace(Identifier),
    /// A call of a built-in, function or module.
    Call(CallStackFrame),
    /// Body (unnamed) scope with locals
    Body(SymbolMap),
}

impl StackFrame {
    /// Get identifier if available or panic.
    pub fn id(&self) -> Option<Identifier> {
        match self {
            StackFrame::Source(id, _) | StackFrame::Namespace(id) => {
                Some(id.clone())
            }
            _ => None,
        }
    }

    /// Print local stack frame
    pub fn print(&self, f: &mut std::fmt::Formatter<'_>, mut depth: usize) -> std::fmt::Result {
        let map = match self {
            StackFrame::Source(id, map) => {
                writeln!(f, "{:depth$}{id} (source):", "")?;
                map
            }
            StackFrame::Namespace(id) => return write!(f, "{:depth$}{id} (namespace)", ""),
            StackFrame::Call(call) => return write!(f, "{:depth$}{name}({args}) (call)", "", args = call.args, name = call.symbol.full_name()),
            StackFrame::Body(map) => map,
        };

        depth += 4;

        for (id, symbol) in map.iter() {
            let full_name = symbol.full_name();
            match &symbol.borrow().def {
                SymbolDefinition::Constant(id, value) => {
                    writeln!(f, "{:depth$}{id} = {value} [{full_name}]", "")?
                }
                _ => writeln!(f, "{:depth$}{id} [{full_name}]", "")?,
            }
        }

        Ok(())
    }
}
