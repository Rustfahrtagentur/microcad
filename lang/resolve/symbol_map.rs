use crate::{eval::*, resolve::*, syntax::*, Id};

/// Map Id to SymbolNode reference
#[derive(Debug, Default)]
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

    /// search for a symbol in symbol map
    pub fn search(&self, name: &QualifiedName) -> EvalResult<SymbolNodeRcMut> {
        trace!("Searching {name} in symbol map");
        let (id, name) = name.split_first();

        if let Some(symbol) = self.get(id.id()) {
            if let Some(symbol) = symbol.borrow().search(&name) {
                Ok(symbol.clone())
            } else {
                Err(EvalError::SymbolNotFound(name))
            }
        } else {
            todo!("error")
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
