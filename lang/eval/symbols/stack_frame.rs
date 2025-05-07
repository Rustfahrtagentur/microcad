// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

/// Frame in [Stack] for *local variables*, *aliases* (*use statements*) and *calls*.
///
/// A *stack frame* can have different types and some provide a storage for *local variables*
/// like [`StackFrame::Source`] and [`StackFrame::Body`]) and some do not, some are named
/// like [`StackFrame::Source`] amd [`StackFrame::Namespace`]) and some do not.
/// [Call] is used for procedural calls.
pub enum StackFrame {
    /// Source file with locals.
    Source(Identifier, SymbolMap),
    /// Namespace scope with locals.
    Namespace(Identifier, SymbolMap),
    /// Module scope with locals.
    ///
    /// Symbol map is built from `ParamterList`.
    Module(Identifier, SymbolMap),
    /// Module initializer scope with locals.
    ModuleInit(SymbolMap),
    /// Body (unnamed) scope with locals.
    Body(SymbolMap),
    /// A call of a built-in, function or module.
    Call {
        /// Symbol that was called.
        symbol: Symbol,
        /// Evaluated call arguments.
        args: CallArgumentValueList,
        /// Source code reference.
        src_ref: SrcRef,
    },
}

impl StackFrame {
    /// Get identifier if available or panic.
    pub fn id(&self) -> Option<Identifier> {
        match self {
            StackFrame::Source(id, _) | StackFrame::Namespace(id, _) => Some(id.clone()),
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
            StackFrame::Module(id, symbol) => {
                return write!(f, "{:depth$}{id} = {symbol} (module)", "");
            }
            StackFrame::ModuleInit(symbol) => {
                return write!(f, "{:depth$} = {symbol} (module)", "");
            }
            StackFrame::Namespace(id, symbol) => {
                return writeln!(f, "{:depth$}{id} = {symbol} (namespace)", "");
            }
            StackFrame::Body(map) => map,
            StackFrame::Call {
                symbol,
                args,
                src_ref: _,
            } => {
                return writeln!(
                    f,
                    "{:depth$}{name}({args}) (call)",
                    "",
                    args = args,
                    name = symbol.full_name()
                );
            }
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

    /// Pretty print single call stack frame.
    pub fn pretty_print(
        &self,
        f: &mut dyn std::fmt::Write,
        source_by_hash: &impl GetSourceByHash,
        idx: usize,
    ) -> std::fmt::Result {
        match self {
            StackFrame::Source(_identifier, _symbol_map) => todo!(),
            StackFrame::Namespace(_identifier, _symbol_map) => todo!(),
            StackFrame::Module(_identifier, _symbol_map) => todo!(),
            StackFrame::ModuleInit(_symbol_map) => todo!(),
            StackFrame::Body(_symbol_map) => todo!(),
            StackFrame::Call {
                symbol,
                args: _,
                src_ref,
            } => {
                writeln!(f, "{:>4}: {name}", idx, name = symbol.full_name())?;

                if let Some(line_col) = src_ref.at() {
                    let source_file = source_by_hash.get_by_hash(src_ref.source_hash());
                    writeln!(
                        f,
                        "            at {filename}:{line_col}",
                        filename = source_file
                            .as_ref()
                            .map(|sf| sf.filename_as_str())
                            .unwrap_or(SourceFile::NO_FILE),
                    )?;
                }
            }
        }

        Ok(())
    }
}
