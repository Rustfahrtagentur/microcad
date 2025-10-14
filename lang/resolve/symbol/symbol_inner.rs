// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{resolve::*, src_ref::*, syntax::*};
use custom_debug::Debug;

/// Symbol content
#[derive(Debug, Clone)]
pub(super) struct SymbolInner {
    /// Symbol definition
    pub(super) def: SymbolDefinition,
    /// Symbol's parent
    #[debug(skip)]
    pub(super) parent: Option<Symbol>,
    /// Symbol's children
    pub(super) children: SymbolMap,
    /// Flag if this symbol has been checked after resolving
    pub(super) checked: bool,
    /// Flag if this symbol was in use
    pub(super) used: std::cell::OnceCell<()>,
}

impl Default for SymbolInner {
    fn default() -> Self {
        Self {
            def: SymbolDefinition::SourceFile(SourceFile::default().into()),
            parent: Default::default(),
            children: Default::default(),
            checked: false,
            used: Default::default(),
        }
    }
}

impl SrcReferrer for SymbolInner {
    fn src_ref(&self) -> SrcRef {
        match &self.def {
            SymbolDefinition::SourceFile(source_file) => source_file.src_ref(),
            SymbolDefinition::Module(module) => module.src_ref(),
            SymbolDefinition::Workbench(workbench) => workbench.src_ref(),
            SymbolDefinition::Function(function) => function.src_ref(),
            SymbolDefinition::Builtin(_) => {
                unreachable!("builtin has no source code reference")
            }
            SymbolDefinition::Constant(_, identifier, _)
            | SymbolDefinition::ConstExpression(_, identifier, _)
            | SymbolDefinition::Argument(identifier, _) => identifier.src_ref(),
            SymbolDefinition::Alias(_, identifier, _) => identifier.src_ref(),
            SymbolDefinition::UseAll(_, name) => name.src_ref(),
            #[cfg(test)]
            SymbolDefinition::Tester(id) => id.src_ref(),
        }
    }
}
