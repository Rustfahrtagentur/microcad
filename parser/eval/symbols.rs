use crate::language::{function::*, identifier::*, module::*, value::*};

#[derive(Clone, Debug)]
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
            Self::BuiltinModule(m) => &m.name,
        }
    }

    pub fn get_symbols(&self, name: &Identifier) -> Vec<&Symbol> {
        match self {
            Self::ModuleDefinition(module) => module.get_symbols_by_name(name),
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
    fn add_symbol(&mut self, symbol: Symbol);
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
    fn add_symbol(&mut self, symbol: Symbol) {
        self.0.push(symbol)
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
