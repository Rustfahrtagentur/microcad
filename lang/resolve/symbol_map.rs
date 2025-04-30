use crate::{eval::*, resolve::*, src_ref::SrcReferrer, syntax::*};

/// Map Id to SymbolNode reference
#[derive(Debug, Default, Clone)]
pub struct SymbolMap(std::collections::btree_map::BTreeMap<Identifier, SymbolNodeRcMut>);

impl std::ops::Deref for SymbolMap {
    type Target = std::collections::btree_map::BTreeMap<Identifier, SymbolNodeRcMut>;

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

    /// Insert a not by it's own id.
    pub fn insert_node(&mut self, symbol: SymbolNodeRcMut) {
        let id = symbol.borrow().id();
        self.0.insert(id, symbol);
    }

    /// Search for a symbol in symbol map.
    pub fn search(&self, name: &QualifiedName) -> EvalResult<SymbolNodeRcMut> {
        if name.is_empty() {
            return Err(EvalError::NotAName(name.src_ref()));
        }

        log::trace!("Searching {name} in symbol map");
        let (id, leftover) = name.split_first();
        if let Some(symbol) = self.get(&id) {
            if leftover.is_empty() {
                return Ok(symbol.clone());
            } else if let Some(symbol) = symbol.borrow().search(&leftover) {
                return Ok(symbol.clone());
            }
        }

        Err(EvalError::SymbolNotFound(name.clone()))
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
