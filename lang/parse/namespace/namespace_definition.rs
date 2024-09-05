//! Namespace definition parser entity

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};


#[derive(Debug)]
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
    fn find_symbols(&self, id: &Id) -> Vec<&Symbol> {
        self.body.find_symbols(id)
    }

    fn add_symbol(&mut self, symbol: Symbol) -> &mut Self {
        self.body.add_symbol(symbol);
        self
    }

    fn copy_symbols<T: Symbols>(&self, into: &mut T) {
        self.body.symbols.iter().for_each(|symbol| {
            into.add_symbol(symbol.clone());
        });
    }
}

impl Parse for std::rc::Rc<NamespaceDefinition> {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let mut pairs = pair.clone().into_inner();
        Ok(std::rc::Rc::new(NamespaceDefinition {
            name: Identifier::parse(pairs.next().unwrap())?,
            body: NamespaceBody::parse(pairs.next().unwrap())?,
            src_ref: pair.into(),
        }))
    }
}

