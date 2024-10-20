// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Namespace definition parser entity

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};

/// Namespace definition
#[derive(Debug, Clone)]
pub struct NamespaceDefinition {
    /// Name of the namespace
    pub name: Identifier,
    /// Namespace body
    pub body: NamespaceBody,
    /// Source code reference
    src_ref: SrcRef,
}

impl NamespaceDefinition {
    /// Create a new namespace definition
    pub fn new(name: Identifier) -> Self {
        Self {
            name,
            body: NamespaceBody::default(),
            src_ref: SrcRef(None),
        }
    }
}

impl SrcReferrer for NamespaceDefinition {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Symbols for NamespaceDefinition {
    fn fetch(&self, id: &Id) -> Option<std::rc::Rc<Symbol>> {
        self.body.fetch(id)
    }

    fn add(&mut self, symbol: Symbol) -> &mut Self {
        self.body.add(symbol);
        self
    }

    fn copy<T: Symbols>(&self, into: &mut T) {
        self.body.symbols.iter().for_each(|(_, symbol)| {
            into.add(symbol.as_ref().clone());
        });
    }
}

impl Parse for std::rc::Rc<NamespaceDefinition> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut pairs = pair.inner();
        Ok(std::rc::Rc::new(NamespaceDefinition {
            name: Identifier::parse(pairs.next().unwrap())?,
            body: NamespaceBody::parse(pairs.next().unwrap())?,
            src_ref: pair.clone().into(),
        }))
    }
}
