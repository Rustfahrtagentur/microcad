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
        self.namespace.add_value(name.into(), value);
        self
    }

    /// Build namespace definition
    pub fn build(&mut self) -> std::rc::Rc<ModuleDefinition> {
        std::rc::Rc::new(self.namespace.clone())
    }
}

impl Symbols for NamespaceBuilder {
    fn fetch_symbols(&self, name: &microcad_core::Id) -> Vec<&Symbol> {
        self.namespace.fetch_symbols(name)
    }

    fn add_symbol(&mut self, symbol: Symbol) -> &mut Self {
        self.namespace.add_symbol(symbol);
        self
    }

    fn copy_symbols<T: Symbols>(&self, into: &mut T) {
        self.namespace.copy_symbols(into)
    }
}

