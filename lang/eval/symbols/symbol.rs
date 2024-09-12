use crate::{eval::*, parse::*};

/// A symbol is a named entity that is used in the
/// symbol table and in the evaluation context to
/// represent a value, a function, a module, etc.
#[derive(Clone, Debug, strum::IntoStaticStr)]
pub enum Symbol {
    /// A value symbol, e.g. a result of an assignment
    Value(Id, Value),
    /// A function symbol, e.g. function a() {}
    Function(std::rc::Rc<FunctionDefinition>),
    /// A module symbol, e.g. module a() {}
    Module(std::rc::Rc<ModuleDefinition>),
    /// A namespace  symbol, e.g. namespace foo {}
    Namespace(std::rc::Rc<NamespaceDefinition>),
    /// A builtin function symbol, e.g. assert()
    BuiltinFunction(BuiltinFunction),
    /// A builtin module symbol, e.g. math::pi
    BuiltinModule(BuiltinModule),
}

impl From<std::rc::Rc<FunctionDefinition>> for Symbol {
    fn from(f: std::rc::Rc<FunctionDefinition>) -> Self {
        Self::Function(f)
    }
}

impl From<std::rc::Rc<ModuleDefinition>> for Symbol {
    fn from(f: std::rc::Rc<ModuleDefinition>) -> Self {
        Self::Module(f)
    }
}

impl From<std::rc::Rc<NamespaceDefinition>> for Symbol {
    fn from(f: std::rc::Rc<NamespaceDefinition>) -> Self {
        Self::Namespace(f)
    }
}

impl From<BuiltinFunction> for Symbol {
    fn from(f: BuiltinFunction) -> Self {
        Self::BuiltinFunction(f)
    }
}

impl From<BuiltinModule> for Symbol {
    fn from(m: BuiltinModule) -> Self {
        Self::BuiltinModule(m)
    }
}

impl Sym for Symbol {
    fn id(&self) -> Option<Id> {
        match self {
            Self::Value(id, _) => Some(id.clone()),
            Self::Function(f) => f.name.id(),
            Self::Module(m) => m.name.id(),
            Self::Namespace(n) => n.name.id(),
            Self::BuiltinFunction(f) => f.name.id(),
            Self::BuiltinModule(m) => m.name.id(),
        }
    }
}

impl Symbol {
    /// fetch all symbols which match id
    pub fn fetch_symbols(&self, name: &Id) -> Vec<&Symbol> {
        match self {
            Self::Module(module) => module.fetch(name),
            Self::Namespace(namespace) => namespace.fetch(name),
            _ => Vec::new(),
        }
    }
}
