// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::{eval::*, parse::*};

/// Module builder
pub struct NamespaceBuilder {
    /// Namespace definition
    namespace: NamespaceDefinition,
}

impl NamespaceBuilder {
    /// Create new module
    pub fn new(name: &str) -> NamespaceBuilder {
        Self {
            namespace: NamespaceDefinition::new(name.into()),
        }
    }

    /// Build namespace definition
    pub fn build(&mut self) -> std::rc::Rc<NamespaceDefinition> {
        std::rc::Rc::new(self.namespace.clone())
    }
}

impl Symbols for NamespaceBuilder {
    fn fetch(&self, name: &microcad_core::Id) -> Option<std::rc::Rc<Symbol>> {
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
