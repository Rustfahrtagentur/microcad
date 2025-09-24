// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model::*};

/// Frame in [Stack] for *local variables*, *aliases* (*use statements*) and *calls*.
///
/// A *stack frame* can have different types and some provide a storage for *local variables*
/// like [`StackFrame::Source`] and [`StackFrame::Body`]) and some do not, some have a *id*
/// like [`StackFrame::Source`] amd [`StackFrame::Module`]) and some do not and
/// [`Call`] is used for procedural calls.
///
/// Each frame store some of these information:
///   - an [`Identifier`]
///   - local variables in a [`SymbolMap`] (e.g. `i = 5;`)
///   - local aliases in a [`SymbolMap`] (e.g. `use std::print;`)
///   - argument value list (e.g. `f(x = 0, y = 1);`
pub enum StackFrame {
    /// Source file with locals.
    Source(Identifier, SymbolMap),
    /// Module scope with locals.
    Module(Identifier, SymbolMap),
    /// initializer scope with locals.
    Init(SymbolMap),
    /// Part scope with locals.
    Workbench(Model, Identifier, SymbolMap),
    /// Body (scope)  with locals.
    Body(SymbolMap),
    /// Function body
    Function(SymbolMap),
    /// A call (e.g. og function or  part).
    Call {
        /// Symbol that was called.
        symbol: Symbol,
        /// Evaluated arguments.
        args: ArgumentValueList,
        /// Source code reference.
        src_ref: SrcRef,
    },
}

impl StackFrame {
    /// Get identifier if available or panic.
    pub fn id(&self) -> Option<Identifier> {
        match self {
            StackFrame::Source(id, _) | StackFrame::Module(id, _) => Some(id.clone()),
            _ => None,
        }
    }

    /// Return symbol of the stack frame, if there is any.
    pub fn symbol(&self) -> Option<Symbol> {
        match &self {
            StackFrame::Call { symbol, .. } => Some(symbol.clone()),
            _ => None,
        }
    }

    /// Return stack frame kind as str
    pub fn kind_str(&self) -> &'static str {
        match self {
            StackFrame::Source(_, _) => "source",
            StackFrame::Module(_, _) => "module",
            StackFrame::Init(_) => "init",
            StackFrame::Workbench(_, _, _) => "workbench",
            StackFrame::Body(_) => "body",
            StackFrame::Function(_) => "function",
            StackFrame::Call {
                symbol: _,
                args: _,
                src_ref: _,
            } => "call",
        }
    }

    /// Print stack frame.
    pub fn print_locals(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        idx: usize,
        mut depth: usize,
    ) -> std::fmt::Result {
        let locals = match self {
            StackFrame::Source(id, locals) => {
                writeln!(f, "{:depth$}[{idx}] Source: {id:?}", "")?;
                locals
            }
            StackFrame::Module(id, locals) => {
                writeln!(f, "{:depth$}[{idx}] Module: {id:?}", "")?;
                locals
            }
            StackFrame::Init(locals) => {
                writeln!(f, "{:depth$}[{idx}] Init", "")?;
                locals
            }
            StackFrame::Workbench(_, id, locals) => {
                writeln!(f, "{:depth$}[{idx}] Workbench: {id:?}", "")?;
                locals
            }
            StackFrame::Body(locals) => {
                writeln!(f, "{:depth$}[{idx}] Body:", "")?;
                locals
            }
            StackFrame::Function(locals) => {
                writeln!(f, "{:depth$}[{idx}] Function:", "")?;
                locals
            }
            StackFrame::Call {
                symbol,
                args,
                src_ref: _,
            } => {
                return writeln!(
                    f,
                    "{:depth$}[{idx}] Call: {name:?}({args})",
                    "",
                    args = args,
                    name = symbol.full_name()
                );
            }
        };

        depth += 4;

        for (id, symbol) in locals.iter() {
            let full_name = symbol.full_name();
            let full_name = if full_name != id.into() {
                format!(" [{full_name}]")
            } else {
                String::new()
            };
            symbol.with_def(|def| match def {
                SymbolDefinition::Constant(visibility, id, value) => writeln!(
                    f,
                    "{:depth$}- {visibility}{id:?} = {value}{full_name} (constant)",
                    ""
                ),
                SymbolDefinition::Argument(id, value) => {
                    writeln!(f, "{:depth$}- {id:?} = {value}{full_name} (argument)", "")
                }
                SymbolDefinition::SourceFile(source) => {
                    writeln!(f, "{:depth$}- {:?} (source)", "", source.filename())
                }
                SymbolDefinition::Module(def) => {
                    writeln!(f, "{:depth$}- {:?}{full_name} (module)", "", def.id)
                }
                SymbolDefinition::Workbench(def) => {
                    writeln!(f, "{:depth$}- {:?}{full_name} (workbench)", "", def.id)
                }
                SymbolDefinition::Function(def) => {
                    writeln!(f, "{:depth$}- {:?}{full_name} (function)", "", def.id)
                }
                SymbolDefinition::Builtin(builtin) => {
                    writeln!(f, "{:depth$}- {:?}{full_name} (builtin)", "", builtin.id)
                }
                SymbolDefinition::Alias(visibility, id, name) => writeln!(
                    f,
                    "{:depth$}- {visibility}{id:?}{full_name} -> {name} (alias)",
                    ""
                ),
                SymbolDefinition::UseAll(visibility, name) => {
                    writeln!(f, "{:depth$}- {visibility}{name}{full_name} (use all)", "")
                }
                #[cfg(test)]
                SymbolDefinition::Tester(id) => writeln!(f, "{:depth$}- {id} (tester)", ""),
            })?
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
            StackFrame::Source(_identifier, _locals) => todo!(),
            StackFrame::Module(_identifier, _locals) => todo!(),
            StackFrame::Init(_locals) => todo!(),
            StackFrame::Workbench(_kind, _identifier, _locals) => todo!(),
            StackFrame::Body(_locals) => todo!(),
            StackFrame::Function(_locals) => todo!(),
            StackFrame::Call {
                symbol,
                args,
                src_ref,
            } => {
                writeln!(f, "{:>4}: {name}({args})", idx, name = symbol.full_name())?;

                if let Some(line_col) = src_ref.at() {
                    let source_file = source_by_hash.get_by_hash(src_ref.source_hash());
                    writeln!(
                        f,
                        "            at {filename}:{line_col}",
                        filename = source_file
                            .as_ref()
                            .map(|sf| sf.filename_as_str())
                            .unwrap_or(crate::invalid!(FILE)),
                    )?;
                }
            }
        }

        Ok(())
    }
}
