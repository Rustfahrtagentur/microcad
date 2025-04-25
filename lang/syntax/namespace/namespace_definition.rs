// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Namespace definition syntax element

use crate::{rc::*, resolve::*, src_ref::*, syntax::*};

/// Namespace definition
#[derive(Debug, Clone)]
pub struct NamespaceDefinition {
    /// Name of the namespace
    pub id: Identifier,
    /// Namespace body
    pub body: Body,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl NamespaceDefinition {
    /// Create a new namespace definition
    pub fn new(name: Identifier) -> Rc<Self> {
        Rc::new(Self {
            id: name,
            body: Body::default(),
            src_ref: SrcRef(None),
        })
    }

    /// Resolve into SymbolNode
    pub fn resolve(self: &Rc<Self>, parent: Option<RcMut<SymbolNode>>) -> RcMut<SymbolNode> {
        let node = SymbolNode::new(SymbolDefinition::Namespace(self.clone()), parent);
        node.borrow_mut().children = self.body.resolve(Some(node.clone()));
        node
    }
}

impl SrcReferrer for NamespaceDefinition {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl PrintSyntax for NamespaceDefinition {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}NamespaceDefinition '{}':", "", self.id)?;
        self.body.print_syntax(f, depth + 1)
    }
}
