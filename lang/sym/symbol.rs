use super::*;
use crate::{
    eval::*,
    parse::*,
    src_ref::{SrcRef, SrcReferrer},
};

/// A symbol is a named entity that is used in the
/// symbol table and in the evaluation context to
/// represent a value, a function, a module, etc.
#[derive(Clone, Debug, Default, strum::IntoStaticStr)]
pub enum Symbol {
    /// An invalid symbol (an error occurred)
    #[default]
    Invalid,
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

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid => write!(f, "Invalid"),
            Self::Value(id, value) => write!(f, "{} = {}", id, value),
            Self::Function(function) => write!(f, "function `{}`", function.name),
            Self::Module(module) => write!(f, "module `{}`", module.name),
            Self::Namespace(namespace) => write!(f, "namespace `{}`", namespace.name),
            Self::BuiltinFunction(function) => write!(f, "{:?}", function),
            Self::BuiltinModule(module) => write!(f, "{:?}", module),
        }
    }
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
        Some(
            match self {
                Self::Invalid => return None,
                Self::Value(id, _) => id,
                Self::Function(f) => f.name.id(),
                Self::Module(m) => m.name.id(),
                Self::Namespace(n) => n.name.id(),
                Self::BuiltinFunction(f) => f.name.id(),
                Self::BuiltinModule(m) => m.name.id(),
            }
            .clone(),
        )
    }
}

impl SrcReferrer for Symbol {
    fn src_ref(&self) -> SrcRef {
        match self {
            Self::Invalid => SrcRef::default(),
            Self::Value(_, value) => value.src_ref(),
            Self::Function(f) => f.src_ref(),
            Self::Module(m) => m.src_ref(),
            Self::Namespace(n) => n.src_ref(),
            Self::BuiltinFunction(_) => SrcRef(None),
            Self::BuiltinModule(_) => SrcRef(None),
        }
    }
}
