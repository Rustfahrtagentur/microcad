// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Statement list syntax element.

use crate::{resolve::*, syntax::*};

/// A list of statements.
#[derive(Clone, Default, Debug)]
pub struct StatementList(pub Vec<Statement>);

impl StatementList {
    /// Fetch all symbols from the statement list.
    pub fn fetch_symbol_map(&self, parent: Option<Symbol>) -> SymbolMap {
        let mut symbol_map = SymbolMap::default();

        // Iterate over all statement fetch definitions
        for statement in &self.0 {
            match statement {
                Statement::Part(m) => {
                    symbol_map.insert(m.id.clone(), m.resolve(parent.clone()));
                }
                Statement::Module(n) => {
                    symbol_map.insert(n.id.clone(), n.resolve(parent.clone()));
                }
                Statement::Function(f) => {
                    symbol_map.insert(f.id.clone(), f.resolve(parent.clone()));
                }
                Statement::Use(u) => symbol_map.append(&mut u.resolve(parent.clone())),
                _ => {}
            }
        }

        symbol_map
    }
}

impl std::ops::Deref for StatementList {
    type Target = Vec<Statement>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for StatementList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in self.iter() {
            writeln!(f, "{statement}")?;
        }
        Ok(())
    }
}
