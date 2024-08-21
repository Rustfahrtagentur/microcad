use crate::{eval::*, language::*};

#[derive(Clone, Debug, strum::IntoStaticStr)]
pub enum Symbol {
    Value(Identifier, Value),
    Function(std::rc::Rc<FunctionDefinition>),
    ModuleDefinition(std::rc::Rc<ModuleDefinition>),
    BuiltinFunction(BuiltinFunction),
    BuiltinModule(BuiltinModule),
}

impl Symbol {
    pub fn name(&self) -> &Identifier {
        match self {
            Self::Value(v, _) => v,
            Self::Function(f) => &f.name,
            Self::ModuleDefinition(m) => &m.name,
            Self::BuiltinFunction(f) => &f.name,
            Self::BuiltinModule(m) => m.name(),
        }
    }

    pub fn get_symbols(&self, name: &Identifier) -> Vec<&Symbol> {
        match self {
            Self::ModuleDefinition(module) => module.find_symbols(name),
            _ => Vec::new(),
        }
    }
}

// @brief Symbol table
/// @details A symbol table is a mapping of symbol
#[derive(Clone, Debug, Default)]
pub struct SymbolTable(Vec<Symbol>);

pub trait Symbols {
    fn find_symbols(&self, name: &Identifier) -> Vec<&Symbol>;
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
    fn add_value(&mut self, id: Identifier, value: Value) -> &mut Self {
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
    fn find_symbols(&self, name: &Identifier) -> Vec<&Symbol> {
        self.0
            .iter()
            .filter(|symbol| symbol.name() == name)
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
