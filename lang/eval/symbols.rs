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
    /// A module definition symbol, e.g. module a() {}, or namespace a {}
    ModuleDefinition(std::rc::Rc<ModuleDefinition>),
    /// A builtin function symbol, e.g. assert()
    BuiltinFunction(BuiltinFunction),
    /// A builtin module symbol, e.g. math::pi
    BuiltinModule(BuiltinModule),
}

pub trait Sym {
    fn id(&self) -> Option<microcad_core::Id>;
}

impl Sym for Symbol {
    fn id(&self) -> Option<Id> {
        match self {
            Self::Value(id, _) => Some(id.clone()),
            Self::Function(f) => f.name.id(),
            Self::ModuleDefinition(m) => m.name.id(),
            Self::BuiltinFunction(f) => f.name.id(),
            Self::BuiltinModule(m) => m.name.id(),
        }
    }
}

impl Symbol {
    pub fn get_symbols(&self, name: &Id) -> Vec<&Symbol> {
        match self {
            Self::ModuleDefinition(module) => module.find_symbols(name),
            _ => Vec::new(),
        }
    }
}

/// Symbol table
///
/// A symbol table is a mapping of symbol
#[derive(Clone, Debug, Default)]
pub struct SymbolTable(Vec<Symbol>);

pub trait Symbols {
    fn find_symbols(&self, id: &Id) -> Vec<&Symbol>;
    fn add_symbol(&mut self, symbol: Symbol) -> &mut Self;
    fn copy_symbols<T: Symbols>(&self, into: &mut T);

    fn add_builtin_function(&mut self, f: BuiltinFunction) -> &mut Self {
        self.add_symbol(Symbol::BuiltinFunction(f));
        self
    }
    fn add_builtin_module(&mut self, m: BuiltinModule) -> &mut Self {
        self.add_symbol(Symbol::BuiltinModule(m));
        self
    }
    fn add_module(&mut self, m: std::rc::Rc<ModuleDefinition>) -> &mut Self {
        self.add_symbol(Symbol::ModuleDefinition(m));
        self
    }
    fn add_value(&mut self, id: Id, value: Value) -> &mut Self {
        self.add_symbol(Symbol::Value(id, value));
        self
    }
    fn add_function(&mut self, f: std::rc::Rc<FunctionDefinition>) -> &mut Self {
        self.add_symbol(Symbol::Function(f));
        self
    }
}

impl SymbolTable {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl Symbols for SymbolTable {
    fn find_symbols(&self, id: &Id) -> Vec<&Symbol> {
        self.0
            .iter()
            .filter(|symbol| {
                if let Some(n) = symbol.id() {
                    n == id
                } else {
                    false
                }
            })
            .collect()
    }
    fn add_symbol(&mut self, symbol: Symbol) -> &mut Self {
        self.0.push(symbol);
        self
    }

    fn copy_symbols<T: Symbols>(&self, into: &mut T) {
        self.0.iter().for_each(|symbol| {
            into.add_symbol(symbol.clone());
        });
    }
}

impl std::ops::Deref for SymbolTable {
    type Target = Vec<Symbol>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SymbolTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
