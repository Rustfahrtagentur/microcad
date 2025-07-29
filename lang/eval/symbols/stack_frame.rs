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
        mut depth: usize,
    ) -> std::fmt::Result {
        let locals = match self {
            StackFrame::Source(id, locals) => {
                writeln!(f, "{:depth$}Source: {id}", "")?;
                locals
            }
            StackFrame::Module(id, locals) => {
                writeln!(f, "{:depth$}Module: {id}", "")?;
                locals
            }
            StackFrame::Init(locals) => {
                writeln!(f, "{:depth$}Init", "")?;
                locals
            }
            StackFrame::Workbench(_, id, locals) => {
                writeln!(f, "{:depth$}Workbench: {id}", "")?;
                locals
            }
            StackFrame::Body(locals) => {
                writeln!(f, "{:depth$}Body", "")?;
                locals
            }
            StackFrame::Function(locals) => {
                writeln!(f, "{:depth$}Function", "")?;
                locals
            }
            StackFrame::Call {
                symbol,
                args,
                src_ref: _,
            } => {
                return writeln!(
                    f,
                    "{:depth$}Call: {name}({args})",
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
            match &symbol.borrow().def {
                SymbolDefinition::Constant(id, value) => writeln!(
                    f,
                    "{:depth$}Constant: {id}: {ty} = {value}{full_name}",
                    "",
                    ty = value.ty()
                )?,
                SymbolDefinition::Argument(id, value) => writeln!(
                    f,
                    "{:depth$}Argument: {id}: {ty} = {value}{full_name}",
                    "",
                    ty = value.ty()
                )?,
                SymbolDefinition::SourceFile(source_file) => {
                    writeln!(f, "{:depth$}Source: {:?}", "", source_file.filename)?
                }
                SymbolDefinition::Module(module_definition) => writeln!(
                    f,
                    "{:depth$}Module: {}{full_name}",
                    "", module_definition.id
                )?,
                SymbolDefinition::External(module_definition) => writeln!(
                    f,
                    "{:depth$}External: {}{full_name}",
                    "", module_definition.id
                )?,
                SymbolDefinition::Workbench(workbench_definition) => writeln!(
                    f,
                    "{:depth$}Workbench: {}{full_name}",
                    "", workbench_definition.id
                )?,
                SymbolDefinition::Function(function_definition) => writeln!(
                    f,
                    "{:depth$}Function: {}{full_name}",
                    "", function_definition.id
                )?,
                SymbolDefinition::Builtin(builtin) => {
                    writeln!(f, "{:depth$}Builtin: {}{full_name}", "", builtin.id)?
                }
                SymbolDefinition::Alias(identifier, qualified_name) => writeln!(
                    f,
                    "{:depth$}Alias: {identifier}{full_name} -> {qualified_name}",
                    ""
                )?,
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
