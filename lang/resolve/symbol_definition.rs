use crate::{parse::*, *};

/// Symbol definition
#[derive(Debug)]
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
        }
    }
}
