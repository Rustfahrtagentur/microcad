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
    /// A module definition symbol, e.g. module a() {}
    ModuleDefinition(std::rc::Rc<ModuleDefinition>),
    /// A namespace definition symbol, e.g. namespace foo {}
    NamespaceDefinition(std::rc::Rc<NamespaceDefinition>),
    /// A builtin function symbol, e.g. assert()
    BuiltinFunction(BuiltinFunction),
    /// A builtin module symbol, e.g. math::pi
    BuiltinModule(BuiltinModule),
}

impl Sym for Symbol {
    fn id(&self) -> Option<Id> {
        match self {
            Self::Value(id, _) => Some(id.clone()),
            Self::Function(f) => f.name.id(),
            Self::ModuleDefinition(m) => m.name.id(),
            Self::NamespaceDefinition(n) => n.name.id(),
            Self::BuiltinFunction(f) => f.name.id(),
            Self::BuiltinModule(m) => m.name.id(),
        }
    }
}

impl Symbol {
    /// fetch all symbols which match id
    pub fn fetch_symbols(&self, name: &Id) -> Vec<&Symbol> {
        match self {
            Self::ModuleDefinition(module) => module.fetch_symbols(name),
            Self::NamespaceDefinition(namespace) => namespace.fetch_symbols(name),
            _ => Vec::new(),
        }
    }
}
