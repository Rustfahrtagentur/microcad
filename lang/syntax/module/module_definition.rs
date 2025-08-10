// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module definition syntax element.

use crate::{rc::*, resolve::*, src_ref::*, syntax::*};

/// Module definition.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModuleDefinition {
    /// Name of the module.
    pub id: Identifier,
    /// Module body.
    pub body: Body,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl ModuleDefinition {
    /// Create a new module definition.
    pub fn new(id: Identifier) -> Rc<Self> {
        Rc::new(Self {
            id,
            body: Body::default(),
            src_ref: SrcRef(None),
        })
    }

    /// Resolve into SymbolNode.
    pub fn resolve(self: &Rc<Self>, parent: Option<Symbol>) -> Symbol {
        let node = Symbol::new(SymbolDefinition::Module(self.clone()), parent);
        node.borrow_mut().children = self.body.resolve(Some(node.clone()));
        node
    }
}

impl SrcReferrer for ModuleDefinition {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl TreeDisplay for ModuleDefinition {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}ModuleDefinition '{}':", "", self.id)?;
        depth.indent();
        self.body.tree_print(f, depth)
    }
}
