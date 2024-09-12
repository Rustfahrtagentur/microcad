use crate::{eval::*, parse::*};

/// Symbol table
///
/// A symbol table is a mapping of symbol
#[derive(Clone, Debug, Default)]
pub struct SymbolTable(Vec<Symbol>);

impl SymbolTable {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl Symbols for SymbolTable {
    fn fetch_symbols(&self, id: &Id) -> Vec<&Symbol> {
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
