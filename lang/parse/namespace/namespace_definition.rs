// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Namespace definition parser entity

use crate::{eval::*, parse::*, parser::*, src_ref::*};

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

    fn add_alias(&mut self, symbol: Symbol, alias: Id) -> &mut Self {
        self.body.add_alias(symbol, alias);
        self
    }

    fn copy<T: Symbols>(&self, into: &mut T) -> EvalResult<()> {
        self.body.symbols.iter().for_each(|(_, symbol)| {
            into.add(symbol.as_ref().clone());
        });
        Ok(())
    }
}

impl Parse for std::rc::Rc<NamespaceDefinition> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut pairs = pair.inner();
        Ok(std::rc::Rc::new(NamespaceDefinition {
            name: Identifier::parse(pairs.next().expect("Identifier expected"))?,
            body: NamespaceBody::parse(pairs.next().expect("NamespaceBody expected"))?,
            src_ref: pair.clone().into(),
        }))
    }
}

impl Eval for std::rc::Rc<NamespaceDefinition> {
    type Output = Symbol;

    fn eval(&self, context: &mut Context) -> EvalResult<Self::Output> {
        let mut namespace = self.as_ref().clone();
        for statement in &self.body.statements {
            match &statement {
                &NamespaceStatement::Assignment(a) => {
                    namespace.add(Symbol::Value(a.name.id().clone(), a.value.eval(context)?));
                }
                NamespaceStatement::FunctionDefinition(f) => {
                    namespace.add(f.clone().into());
                }
                NamespaceStatement::ModuleDefinition(m) => {
                    namespace.add(m.clone().into());
                }
                NamespaceStatement::NamespaceDefinition(n) => {
                    let n = n.eval(context)?;
                    namespace.add(n);
                }
                NamespaceStatement::Use(u) => {
                    if let Some(symbols) = u.eval(context)? {
                        for (id, symbol) in symbols.iter() {
                            namespace.add_alias(symbol.as_ref().clone(), id.clone());
                        }
                    }
                }
            }
        }

        Ok(Symbol::Namespace(std::rc::Rc::new(namespace)))
    }
}
