// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, rc::*, resolve::*, syntax::*, value::*};

/// Symbol definition
#[derive(Debug, Clone)]
pub enum SymbolDefinition {
    /// Source file symbol.
    SourceFile(Rc<SourceFile>),
    /// Namespace symbol.
    Namespace(Rc<NamespaceDefinition>),
    /// External namespace symbol (not already loaded).
    External(Rc<NamespaceDefinition>),
    /// Module symbol.
    Module(Rc<ModuleDefinition>),
    /// Function symbol.
    Function(Rc<FunctionDefinition>),
    /// Builtin symbol.
    Builtin(Rc<Builtin>),
    /// Constant.
    Constant(Identifier, Value),
    /// Call argument value.
    CallArgument(Identifier, Value),
    /// Alias of a pub use statement.
    Alias(Identifier, QualifiedName),
}

impl SymbolDefinition {
    /// Returns ID of this definition.
    pub fn id(&self) -> Identifier {
        match &self {
            Self::Namespace(n) | Self::External(n) => n.id.clone(),
            Self::Module(m) => m.id.clone(),
            Self::Function(f) => f.id.clone(),
            Self::SourceFile(s) => s.id(),
            Self::Builtin(m) => m.id(),
            Self::Constant(id, _) | Self::CallArgument(id, _) | Self::Alias(id, _) => id.clone(),
        }
    }

    /// Resolve into SymbolNode.
    pub fn resolve(&self, parent: Option<Symbol>) -> Symbol {
        match self {
            Self::Module(m) => m.resolve(parent),
            Self::Namespace(n) => n.resolve(parent),
            Self::Function(f) => f.resolve(parent),
            Self::SourceFile(s) => s.resolve(parent),
            Self::External(e) => unreachable!("external {} must be loaded first", e.id),
            // A builtin symbols and constants cannot have child symbols,
            // hence the resolve trait does not need to be implemented
            symbol_definition => Symbol::new(symbol_definition.clone(), parent),
        }
    }
}

impl std::fmt::Display for SymbolDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Module(_) => write!(f, "(module)"),
            Self::Namespace(_) => write!(f, "(namespace)"),
            Self::External(_) => write!(f, "(external)"),
            Self::Function(_) => write!(f, "(function)"),
            Self::SourceFile(_) => write!(f, "(file)"),
            Self::Builtin(_) => write!(f, "(builtin)"),
            Self::Constant(_, value) => write!(f, "(constant) = {value}"),
            Self::CallArgument(_, value) => write!(f, "(call_argument) = {value}"),
            Self::Alias(_, name) => write!(f, "(alias) => {name}"),
        }
    }
}
