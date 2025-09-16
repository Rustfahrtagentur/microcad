// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{builtin::*, rc::*, syntax::*, value::*};

/// Symbol definition
#[derive(Debug, Clone)]
pub enum SymbolDefinition {
    /// Source file symbol.
    SourceFile(Rc<SourceFile>),
    /// Module symbol.
    Module(Rc<ModuleDefinition>),
    /// Part symbol.
    Workbench(Rc<WorkbenchDefinition>),
    /// Function symbol.
    Function(Rc<FunctionDefinition>),
    /// Builtin symbol.
    Builtin(Rc<Builtin>),
    /// Constant.
    Constant(Visibility, Identifier, Value),
    /// Argument value.
    Argument(Identifier, Value),
    /// Alias of a pub use statement.
    Alias(Visibility, Identifier, QualifiedName),
    /// Use all available symbols in the module with the given name.
    UseAll(Visibility, QualifiedName),
    /// Just a dummy for testing
    #[cfg(test)]
    Tester(Identifier),
}

impl SymbolDefinition {
    /// Returns ID of this definition.
    pub fn id(&self) -> Identifier {
        match &self {
            Self::Workbench(w) => w.id.clone(),
            Self::Module(m) => m.id.clone(),
            Self::Function(f) => f.id.clone(),
            Self::SourceFile(s) => s.id(),
            Self::Builtin(m) => m.id(),
            Self::Constant(_, id, _) | Self::Argument(id, _) | Self::Alias(_, id, _) => id.clone(),
            Self::UseAll(..) => Identifier::none(),
            #[cfg(test)]
            Self::Tester(id) => id.clone(),
        }
    }
}

impl std::fmt::Display for SymbolDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Workbench(w) => write!(f, "({})", w.kind),
            Self::Module(..) => write!(f, "(module)"),
            Self::Function(..) => write!(f, "(function)"),
            Self::SourceFile(..) => write!(f, "(file)"),
            Self::Builtin(..) => write!(f, "(builtin)"),
            Self::Constant(.., value) => write!(f, "(constant) = {value}"),
            Self::Argument(.., value) => write!(f, "(call_argument) = {value}"),
            Self::Alias(.., name) => write!(f, "(alias) => {name}"),
            Self::UseAll(.., name) => write!(f, "(use all) => {name}"),
            #[cfg(test)]
            Self::Tester(id) => write!(f, "(tester) => {id}"),
        }
    }
}
