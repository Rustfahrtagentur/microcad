// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Statement list syntax element.

use crate::{resolve::*, syntax::*};
use derive_more::Deref;

/// A list of statements.
#[derive(
    Clone,
    Default,
    Debug,
    Deref,
    bincode::Encode,
    bincode::Decode,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct StatementList(pub Vec<Statement>);

impl StatementList {
    /// Fetch all symbols from the statement list.
    pub fn fetch_symbol_map(&self, parent: Option<Symbol>) -> SymbolMap {
        let mut symbol_map = SymbolMap::default();

        // Iterate over all statement fetch definitions
        for statement in &self.0 {
            match statement {
                Statement::Workbench(w) => {
                    symbol_map.insert(w.id.clone(), w.resolve(parent.clone()));
                }
                Statement::Module(n) => {
                    symbol_map.insert(n.id.clone(), n.resolve(parent.clone()));
                }
                Statement::Function(f) => {
                    symbol_map.insert(f.id.clone(), f.resolve(parent.clone()));
                }
                Statement::Use(u) => {
                    if let Some((id, symbol)) = u.resolve(parent.clone()) {
                        symbol_map.insert(id, symbol);
                    }
                }
                Statement::Init(_)
                | Statement::Return(_)
                | Statement::If(_)
                | Statement::InnerAttribute(_)
                | Statement::Assignment(_)
                | Statement::Expression(_) => {}
            }
        }

        symbol_map
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
