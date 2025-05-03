// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, resolve::SymbolNodeRcMut};
use std::collections::BTreeMap;

/// A stack frame is map of local variables.
pub enum LocalFrame {
    /// Source file with locals.
    Source(Identifier, BTreeMap<Identifier, SymbolNodeRcMut>),
    /// Namespace scope without locals
    Namespace(Identifier),
    /// Module scope without locals
    Module(Identifier),
    /// Standard (unnamed) scope with locals
    Scope(BTreeMap<Identifier, SymbolNodeRcMut>),
}

impl LocalFrame {
    /// Get identifier if available or panic.
    pub fn id(&self) -> Option<Identifier> {
        match self {
            LocalFrame::Source(id, _) | LocalFrame::Namespace(id) | LocalFrame::Module(id) => {
                Some(id.clone())
            }
            _ => None,
        }
    }

    /// Print local stack frame
    pub fn print(&self, f: &mut std::fmt::Formatter<'_>, mut depth: usize) -> std::fmt::Result {
        let map = match self {
            LocalFrame::Source(id, map) => {
                writeln!(f, "{:depth$}{id} (source):", "")?;
                map
            }
            LocalFrame::Namespace(id) => return write!(f, "{:depth$}{id} (namespace)", ""),
            LocalFrame::Module(id) => return write!(f, "{:depth$}{id} (module)", ""),
            LocalFrame::Scope(map) => map,
        };

        depth += 4;

        for (id, symbol) in map.iter() {
            let full_name = symbol.borrow().full_name();
            match &symbol.borrow().def {
                SymbolDefinition::Constant(id, value) => {
                    writeln!(f, "{:depth$}{id} = {value} [{full_name}]", "")?
                }
                _ => writeln!(f, "{:depth$}{id} [{full_name}]", "")?,
            }
        }

        Ok(())
    }
}
