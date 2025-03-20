use crate::{Id, parse::*};
use std::ops::Deref;

/// Qualified name of a symbol
pub struct SymbolPath(Vec<Id>);

impl SymbolPath {
    /// remove the first name from path
    pub fn remove_first(&self) -> Self {
        Self(self.0[1..].to_vec())
    }
}

impl From<QualifiedName> for SymbolPath {
    fn from(name: QualifiedName) -> Self {
        Self(name.0.iter().map(|n| n.id().clone()).collect::<Vec<_>>())
    }
}

impl From<&str> for SymbolPath {
    fn from(name: &str) -> Self {
        Self(name.split("::").map(|n| Id::new(n)).collect::<Vec<_>>())
    }
}

impl Deref for SymbolPath {
    type Target = Vec<Id>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
