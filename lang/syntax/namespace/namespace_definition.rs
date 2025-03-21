// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Namespace definition syntax element

use crate::{Rc, src_ref::*, syntax::*};

/// Namespace definition
#[derive(Debug, Clone)]
pub struct NamespaceDefinition {
    /// Name of the namespace
    pub name: Identifier,
    /// Namespace body
    pub body: Body,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl NamespaceDefinition {
    /// Create a new namespace definition
    pub fn new(name: Identifier) -> Rc<Self> {
        Rc::new(Self {
            name,
            body: Body::default(),
            src_ref: SrcRef(None),
        })
    }
}

impl SrcReferrer for NamespaceDefinition {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl PrintSyntax for NamespaceDefinition {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}NamespaceDefinition '{}':", "", self.name)?;
        self.body.print_syntax(f, depth + 1)
    }
}
