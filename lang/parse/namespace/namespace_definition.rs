// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Namespace definition parser entity

use crate::{parse::*, parser::*, src_ref::*};

/// Namespace definition
#[derive(Debug, Clone)]
pub struct NamespaceDefinition {
    /// Name of the namespace
    pub name: Identifier,
    /// Namespace body
    pub body: Body,
    /// Source code reference
    src_ref: SrcRef,
}

impl NamespaceDefinition {
    /// Create a new namespace definition
    pub fn new(name: Identifier) -> Self {
        Self {
            name,
            body: Body::default(),
            src_ref: SrcRef(None),
        }
    }
}

impl SrcReferrer for NamespaceDefinition {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Parse for std::rc::Rc<NamespaceDefinition> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut pairs = pair.inner();
        Ok(std::rc::Rc::new(NamespaceDefinition {
            name: Identifier::parse(pairs.next().expect("Identifier expected"))?,
            body: Body::parse(pairs.next().expect("NamespaceBody expected"))?,
            src_ref: pair.clone().into(),
        }))
    }
}

impl Syntax for NamespaceDefinition {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}NamespaceDefinition '{}'", "", self.name)?;
        self.body.print_syntax(f, depth + 1)
    }
}
