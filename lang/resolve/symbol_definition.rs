// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, rc::*, syntax::*, value::*};

/// Symbol definition
#[derive(Debug, Clone, bincode::Encode, bincode::Decode, serde::Serialize, serde::Deserialize)]
pub enum SymbolDefinition {
    /// Source file symbol.
    SourceFile(Rc<SourceFile>),
    /// Module symbol.
    Module(Rc<ModuleDefinition>),
    /// External module symbol (not already loaded).
    External(Rc<ModuleDefinition>),
    /// Part symbol.
    Workbench(Rc<WorkbenchDefinition>),
    /// Function symbol.
    Function(Rc<FunctionDefinition>),
    /// Builtin symbol.
    #[serde(skip)]
    Builtin(Rc<Builtin>),
    /// Constant.
    Constant(Identifier, Value),
    /// Argument value.
    Argument(Identifier, Value),
    /// Alias of a pub use statement.
    Alias(Identifier, QualifiedName),
    /// Use all available symbols in the module with the given name.
    UseAll(QualifiedName),
}

impl SymbolDefinition {
    /// Returns ID of this definition.
    pub fn id(&self) -> Identifier {
        match &self {
            Self::Workbench(w) => w.id.clone(),
            Self::Module(m) | Self::External(m) => m.id.clone(),
            Self::Function(f) => f.id.clone(),
            Self::SourceFile(s) => s.id(),
            Self::Builtin(m) => m.id(),
            Self::Constant(id, _) | Self::Argument(id, _) | Self::Alias(id, _) => id.clone(),
            Self::UseAll(_) => Identifier::none(),
        }
    }
}

impl std::fmt::Display for SymbolDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Workbench(w) => write!(f, "({})", w.kind),
            Self::Module(_) => write!(f, "(module)"),
            Self::External(_) => write!(f, "(external)"),
            Self::Function(_) => write!(f, "(function)"),
            Self::SourceFile(_) => write!(f, "(file)"),
            Self::Builtin(_) => write!(f, "(builtin)"),
            Self::Constant(_, value) => write!(f, "(constant) = {value}"),
            Self::Argument(_, value) => write!(f, "(call_argument) = {value}"),
            Self::Alias(_, name) => write!(f, "(alias) => {name}"),
            Self::UseAll(name) => write!(f, "(use all) => {name}"),
        }
    }
}
