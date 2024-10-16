use crate::eval::*;

/// Symbol table
///
/// A symbol table is a mapping of symbol
#[derive(Clone, Debug, Default)]
pub struct SymbolTable(std::collections::HashMap<Id, std::rc::Rc<Symbol>>);

impl Symbols for SymbolTable {
    fn fetch(&self, id: &Id) -> Option<std::rc::Rc<Symbol>> {        
        match self.0.get(id) {
            Some(symbol) => Some(symbol.clone()),
            None => None,
        }
    }

    fn add(&mut self, symbol: Symbol) -> &mut Self {
        self.0.insert(symbol.id().unwrap(), std::rc::Rc::new(symbol));
        self
    }

    fn copy<T: Symbols>(&self, into: &mut T) {
        self.0.iter().for_each(|(_, symbol)| {
            into.add(symbol.as_ref().clone());
        });
    }
}

impl std::ops::Deref for SymbolTable {
    type Target = std::collections::HashMap<Id, std::rc::Rc<Symbol>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SymbolTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
