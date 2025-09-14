// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module definition syntax element.

use crate::{rc::*, src_ref::*, syntax::*};

/// Module definition.
#[derive(Debug, Clone)]
pub struct ModuleDefinition {
    /// Visibility of the module.
    pub visibility: Visibility,
    /// Name of the module.
    pub id: Identifier,
    /// Module body.
    pub body: Body,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl ModuleDefinition {
    /// Create a new module definition.
    pub fn new(visibility: Visibility, id: Identifier) -> Rc<Self> {
        Rc::new(Self {
            visibility,
            id,
            body: Body::default(),
            src_ref: SrcRef(None),
        })
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
