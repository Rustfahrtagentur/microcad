use crate::{Id, eval::*, resolve::*, syntax::*};

/// Map Id to SymbolNode reference
#[derive(Debug, Default)]
pub struct SymbolMap(std::collections::btree_map::BTreeMap<Id, RcMut<SymbolNode>>);

impl std::ops::Deref for SymbolMap {
    type Target = std::collections::btree_map::BTreeMap<Id, RcMut<SymbolNode>>;

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
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn search(&self, name: &QualifiedName) -> EvalResult<RcMut<SymbolNode>> {
        let (id, name) = name.split_first();

        if let Some(symbol) = self.get(id.id()) {
            if let Some(symbol) = symbol.borrow().search_down(&name) {
                return Ok(symbol.clone());
            } else {
                todo!("error")
            }
        }

        todo!("error")
    }
}

impl std::fmt::Display for SymbolMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (id, symbol) in self.0.iter() {
            write!(f, "{symbol}")?;
        }

        Ok(())
    }
}
