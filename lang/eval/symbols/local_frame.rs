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
    pub fn print(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
        let (map, depth) = match self {
            LocalFrame::Source(id, map) => {
                writeln!(f, "{:depth$}{id} (source):", "")?;
                (map, depth + 2)
            }
            LocalFrame::Namespace(id) => return write!(f, "{:depth$}{id} (namespace)", ""),
            LocalFrame::Module(id) => return write!(f, "{:depth$}{id} (module)", ""),
            LocalFrame::Scope(map) => (map, depth),
        };

        for (id, local) in map.iter() {
            writeln!(f, "{:depth$}{id} [{}]", "", local.borrow().full_name())?
        }

        Ok(())
    }
}
