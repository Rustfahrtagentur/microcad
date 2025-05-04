// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::{resolve::*, syntax::Identifier};

/// Builder pattern to build builtin namespaces
pub struct NamespaceBuilder {
    // Namespace symbol
    namespace: Symbol,
}

impl NamespaceBuilder {
    /// Create new namespace symbol with a name
    pub fn new(id: Identifier) -> Self {
        Self {
            namespace: Symbol::new_namespace(id),
        }
    }

    /// Add a symbol to the namespace
    pub fn symbol(self, symbol: Symbol) -> Self {
        Symbol::add_child(&self.namespace, symbol);
        self
    }

    /// Return our namespace symbol
    pub fn build(self) -> Symbol {
        self.namespace
    }
}
