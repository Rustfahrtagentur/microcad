// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::{resolve::*, syntax::Identifier};

/// Builder pattern to build builtin namespaces
pub struct NamespaceBuilder {
    // Namespace symbol
    namespace: SymbolNodeRcMut,
}

impl NamespaceBuilder {
    /// Create new namespace symbol with a name
    pub fn new(id: Identifier) -> Self {
        Self {
            namespace: SymbolNode::new_namespace(id),
        }
    }

    /// Add a symbol to the namespace
    pub fn symbol(self, symbol: SymbolNodeRcMut) -> Self {
        SymbolNode::insert_child(&self.namespace, symbol);
        self
    }

    /// Return our namespace symbol
    pub fn build(self) -> SymbolNodeRcMut {
        self.namespace
    }
}
