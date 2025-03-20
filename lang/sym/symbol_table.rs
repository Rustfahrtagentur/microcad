use super::*;

/// Symbol table
///
/// A symbol table is a mapping of symbol
#[derive(Clone, Debug, Default)]
pub struct SymbolTable(std::collections::HashMap<Id, Rc<Symbol>>);

impl SymbolTable {
    /// Merge two symbol tables
    ///
    /// This function merges two symbol tables into one.
    pub fn merge(&mut self, other: &mut Self) {
        other.0.iter().for_each(|(id, symbol)| {
            if self.0.contains_key(id) {
                panic!("Symbol with id `{}` already exists", id); // TODO Better error handling on symbol name collision
            }
            self.0.insert(id.clone(), symbol.clone());
        });
    }
}

impl Symbols for SymbolTable {
    fn fetch(&self, id: &Id) -> Option<Rc<Symbol>> {
        self.0.get(id).cloned()
    }

    fn add(&mut self, symbol: Symbol) -> &mut Self {
        match symbol {
            Symbol::Invalid => {}
            _ => {
                self.0
                    .insert(symbol.id().expect("Symbol id is not set"), Rc::new(symbol));
            }
        }

        self
    }

    fn add_alias(&mut self, symbol: Symbol, alias: Id) -> &mut Self {
        match symbol {
            Symbol::Invalid => {}
            _ => {
                self.0.insert(alias, Rc::new(symbol));
            }
        }

        self
    }

    fn copy<T: Symbols>(&self, into: &mut T) -> SymResult<()> {
        self.0.iter().for_each(|(_, symbol)| {
            into.add(symbol.as_ref().clone());
        });
        Ok(())
    }
}

impl std::ops::Deref for SymbolTable {
    type Target = std::collections::HashMap<Id, Rc<Symbol>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SymbolTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
