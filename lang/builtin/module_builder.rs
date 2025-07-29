// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builder pattern to build built-in modules.

use crate::{resolve::*, syntax::*};

/// Builder pattern to build built-in modules.
pub struct ModuleBuilder {
    // Symbol to build.
    module: Symbol,
}

impl ModuleBuilder {
    /// Create new module symbol with a name.
    pub fn new(id: Identifier) -> Self {
        Self {
            module: Symbol::new_module(id),
        }
    }

    /// Add a symbol to the module.
    pub fn symbol(self, symbol: Symbol) -> Self {
        Symbol::add_child(&self.module, symbol);
        self
    }

    /// Return our module symbol.
    pub fn build(self) -> Symbol {
        self.module
    }
}
