// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

/// Frame in [Stack] for *local variables*, *aliases* (*use statements*) and *calls*.
///
/// A *stack frame* can have different types and some provide a storage for *local variables*
/// like [`StackFrame::Source`] and [`StackFrame::Body`]) and some do not, some have a *id*
/// like [`StackFrame::Source`] amd [`StackFrame::Namespace`]) and some do not and
/// [`Call`] is used for procedural calls.
///
/// Each frame store some of these information:
///   - an [`Identifier`]
///   - local variables in a [`SymbolMap`] (e.g. `i = 5;`)
///   - local aliases in a [`SymbolMap`] (e.g. `use std::print;`)
///   - call argument value list (e.g. `f(x = 0, y = 1);`
pub enum StackFrame {
    /// Source file with locals.
    Source(Identifier, SymbolMap),
    /// Namespace scope with locals.
    Namespace(Identifier, SymbolMap),
    /// Module scope with locals.
    ///
    /// Symbol map is built from [`ParameterList`].
    Module(Identifier, SymbolMap),
    /// Module initializer scope with locals.
    ///
    /// Symbol map is built from [`ParameterList`].
    ModuleInit(SymbolMap),
    /// Body (scope)  with locals.
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

    /// Print stack frame.
    pub fn print_locals(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        mut depth: usize,
    ) -> std::fmt::Result {
        let map = match self {
            StackFrame::Source(id, map) => {
                writeln!(f, "{:depth$}{id} (source):", "")?;
                map
            }
            StackFrame::Module(id, symbols) => {
                writeln!(f, "{:depth$}{id} (module)", "")?;
                return symbols.print(f, depth + 4);
            }
            StackFrame::ModuleInit(symbols) => {
                writeln!(f, "{:depth$} (module init):", "")?;
                return symbols.print(f, depth + 4);
            }
            StackFrame::Namespace(id, symbols) => {
                writeln!(f, "{:depth$}{id} (namespace):", "")?;
                return symbols.print(f, depth + 4);
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
    pub fn print_stack(
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
