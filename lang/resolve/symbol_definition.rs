use crate::{Id, eval::*, rc_mut::*, syntax::*, value::Value};

/// Symbol definition
#[derive(Debug, Clone)]
pub enum SymbolDefinition {
    /// Source file symbol
    SourceFile(Rc<SourceFile>),
    /// Namespace symbol
    Namespace(Rc<NamespaceDefinition>),
    /// Module symbol
    Module(Rc<ModuleDefinition>),
    /// Function symbol
    Function(Rc<FunctionDefinition>),
    /// Builtin function symbol
    BuiltinFunction(Rc<BuiltinFunction>),
    /// Builtin module symbol
    BuiltinModule(Rc<BuiltinModule>),
    /// Builtin constant
    Constant(Id, Value),
}

impl SymbolDefinition {
    /// Returns ID of this definition
    pub fn id(&self) -> Id {
        match &self {
            Self::Namespace(n) => n.name.id().clone(),
            Self::Module(m) => m.name.id().clone(),
            Self::Function(f) => f.name.id().clone(),
            Self::SourceFile(s) => s.filename_as_str().into(),
            Self::BuiltinFunction(f) => f.id.clone(),
            Self::BuiltinModule(m) => m.id.clone(),
            Self::Constant(id, _) => id.clone(),
        }
    }
}

impl std::fmt::Display for SymbolDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = self.id();
        match self {
            Self::Module(_) => write!(f, "module {}", id),
            Self::Namespace(_) => write!(f, "namespace {}", id),
            Self::Function(_) => write!(f, "function {}", id),
            Self::SourceFile(_) => write!(f, "file {}", id),
            Self::BuiltinFunction(_) => write!(f, "builtin_fn {}", id),
            Self::BuiltinModule(_) => write!(f, "builtin_mod {}", id),
            Self::Constant(id, value) => writeln!(f, "const {} = {}", id, value),
        }
    }
}
