use crate::{eval::*, rc::*, resolve::*, syntax::*, value::*, Id};

/// Symbol definition
#[derive(Debug, Clone)]
pub enum SymbolDefinition {
    /// Source file symbol
    SourceFile(Rc<SourceFile>),
    /// Namespace symbol
    Namespace(Rc<NamespaceDefinition>),
    /// External namespace symbol (not already loaded)
    External(Rc<NamespaceDefinition>),
    /// Module symbol
    Module(Rc<ModuleDefinition>),
    /// Function symbol
    Function(Rc<FunctionDefinition>),
    /// Builtin function symbol
    BuiltinFunction(Rc<BuiltinFunction>),
    /// Builtin module symbol
    BuiltinModule(Rc<BuiltinModule>),
    /// Constant
    Constant(Identifier, Value),
    /// Alias of a pub use statement
    Alias(Identifier, QualifiedName),
}

impl SymbolDefinition {
    /// Returns ID of this definition.
    pub fn id(&self) -> Id {
        match &self {
            Self::Namespace(n) | Self::External(n) => n.id.id().clone(),
            Self::Module(m) => m.id.id().clone(),
            Self::Function(f) => f.id.id().clone(),
            Self::SourceFile(s) => s.id().id().clone(),
            Self::BuiltinFunction(f) => f.id.clone(),
            Self::BuiltinModule(m) => m.id.clone(),
            Self::Constant(id, _) => id.id().clone(),
            Self::Alias(id, _) => id.id().clone(),
        }
    }

    /// Resolve into SymbolNode.
    pub fn resolve(&self, parent: Option<RcMut<SymbolNode>>) -> RcMut<SymbolNode> {
        match self {
            Self::Module(m) => m.resolve(parent),
            Self::Namespace(n) => n.resolve(parent),
            Self::Function(f) => f.resolve(parent),
            Self::SourceFile(s) => s.resolve(parent),
            Self::External(e) => unreachable!("external {} must be loaded first", e.id),
            // A builtin symbols and constants cannot have child symbols,
            // hence the resolve trait does not need to be implemented
            symbol_definition => SymbolNode::new(symbol_definition.clone(), parent),
        }
    }
}

impl std::fmt::Display for SymbolDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = self.id();
        match self {
            Self::Module(_) => write!(f, "{} (module)", id),
            Self::Namespace(_) => write!(f, "{} (namespace)", id),
            Self::External(_) => write!(f, "{} (external)", id),
            Self::Function(_) => write!(f, "{} (function)", id),
            Self::SourceFile(_) => write!(f, "{} (file)", id),
            Self::BuiltinFunction(_) => write!(f, "{} (builtin function)", id),
            Self::BuiltinModule(_) => write!(f, "{} (builtin module)", id),
            Self::Constant(id, value) => write!(f, "{} (constant) = {}", id, value),
            Self::Alias(id, name) => write!(f, "{} (alias) => {}", id, name),
        }
    }
}
