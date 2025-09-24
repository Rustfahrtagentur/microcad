// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module definition syntax element.

use crate::{rc::*, src_ref::*, syntax::*};

/// Module definition.
#[derive(Debug, Clone, Default)]
pub struct ModuleDefinition {
    /// Visibility of the module.
    pub visibility: Visibility,
    /// Name of the module.
    pub id: Identifier,
    /// Module body. ('None' if external module
    pub body: Option<Body>,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl ModuleDefinition {
    /// Create a new module definition.
    pub fn new(visibility: Visibility, id: Identifier) -> Rc<Self> {
        Rc::new(Self {
            visibility,
            id,
            ..Default::default()
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
        if let Some(body) = &self.body {
            writeln!(f, "{:depth$}ModuleDefinition '{}':", "", self.id)?;
            depth.indent();
            body.tree_print(f, depth)
        } else {
            writeln!(f, "{:depth$}ModuleDefinition '{}' (external)", "", self.id)
        }
    }
}
