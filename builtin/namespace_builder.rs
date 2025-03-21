// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::*;

/// Builder pattern to build builtin namespaces
pub struct NamespaceBuilder {
    // Namespace symbol
    namespace: RcMut<SymbolNode>,
}

impl NamespaceBuilder {
    /// Create new namespace symbol with a name
    pub fn new(id: &str) -> Self {
        Self {
            namespace: SymbolNode::new_builtin_namespace(id),
        }
    }

    /// Add a symbol to the namespace
    pub fn symbol(mut self, symbol: RcMut<SymbolNode>) -> Self {
        SymbolNode::insert_child(&mut self.namespace, symbol);
        self
    }

    /// Return our namespace symbol
    pub fn build(self) -> RcMut<SymbolNode> {
        self.namespace
    }
}
