use crate::{eval::*, resolve::*, syntax::*, Id};

/// Map Id to SymbolNode reference
#[derive(Debug, Default, Clone)]
pub struct SymbolMap(std::collections::btree_map::BTreeMap<Id, SymbolNodeRcMut>);

impl std::ops::Deref for SymbolMap {
    type Target = std::collections::btree_map::BTreeMap<Id, SymbolNodeRcMut>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SymbolMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl SymbolMap {
    /// Create symbol new map
    pub fn new() -> Self {
        Self(Default::default())
    }

    /// insert a not by it's own id
    pub fn insert_node(&mut self, symbol: SymbolNodeRcMut) {
        let id = symbol.borrow().id();
        self.0.insert(id, symbol);
    }

    /// search for a symbol in symbol map
    pub fn search(&self, name: &QualifiedName) -> EvalResult<SymbolNodeRcMut> {
        log::trace!("Searching {name} in symbol map");
        let (id, name) = name.split_first();

        if let Some(symbol) = self.get(id.id()) {
            if let Some(symbol) = symbol.borrow().search(&name) {
                Ok(symbol.clone())
            } else {
                Err(EvalError::SymbolNotFound(name))
            }
        } else {
            Err(EvalError::SymbolNotFound(name))
        }
    }
}

impl std::fmt::Display for SymbolMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (_, symbol) in self.0.iter() {
            write!(f, "{symbol}")?;
        }

        Ok(())
    }
}
