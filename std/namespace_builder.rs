// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::{eval::*, parse::*};

/// Module builder
pub struct NamespaceBuilder {
    /// Namespace definition
    namespace: ModuleDefinition,
}

impl NamespaceBuilder {
    /// Create new module
    pub fn new(name: &str) -> NamespaceBuilder {
        Self {
            namespace: ModuleDefinition::new(name.into()),
        }
    }

    /// Add a value
    #[cfg(test)]
    pub fn add_value(&mut self, name: &str, value: Value) -> &mut Self {
        self.namespace.add(Symbol::Value(name.into(), value));
        self
    }

    /// Build namespace definition
    pub fn build(&mut self) -> std::rc::Rc<ModuleDefinition> {
        std::rc::Rc::new(self.namespace.clone())
    }
}

impl Symbols for NamespaceBuilder {
    fn fetch(&self, name: &microcad_core::Id) -> Vec<&Symbol> {
        self.namespace.fetch(name)
    }

    fn add(&mut self, symbol: Symbol) -> &mut Self {
        self.namespace.add(symbol);
        self
    }

    fn copy<T: Symbols>(&self, into: &mut T) {
        self.namespace.copy(into)
    }
}
