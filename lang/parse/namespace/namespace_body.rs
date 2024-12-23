// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Namespace body parser entity

use crate::{eval::*, parse::*, parser::*, src_ref::*, sym::*};

/// Namespace body
#[derive(Debug, Default, Clone)]
pub struct NamespaceBody {
    /// Namespace statements
    pub statements: Vec<NamespaceStatement>,
    /// Namespace's local symbol table
    pub symbols: SymbolTable,
    /// Source code reference
    src_ref: SrcRef,
}

impl NamespaceBody {
    /// Add statement to namespace
    pub fn add_statement(&mut self, statement: NamespaceStatement) {
        self.statements.push(statement.clone());
        match statement {
            NamespaceStatement::FunctionDefinition(function) => {
                self.add(function.into());
            }
            NamespaceStatement::NamespaceDefinition(namespace) => {
                self.add(namespace.into());
            }
            NamespaceStatement::ModuleDefinition(module) => {
                self.add(module.into());
            }
            _ => {}
        }
    }
}

impl Symbols for NamespaceBody {
    fn fetch(&self, id: &Id) -> Option<std::rc::Rc<Symbol>> {
        self.symbols.fetch(id)
    }

    fn add(&mut self, symbol: Symbol) -> &mut Self {
        self.symbols.add(symbol);
        self
    }

    fn add_alias(&mut self, symbol: Symbol, alias: Id) -> &mut Self {
        self.symbols.add_alias(symbol, alias);
        self
    }

    fn copy<T: Symbols>(&self, into: &mut T) -> SymResult<()> {
        self.symbols.copy(into)
    }
}

impl Parse for NamespaceBody {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut body = NamespaceBody::default();
        for pair in pair.inner() {
            if pair.as_rule() == Rule::namespace_statement {
                body.add_statement(NamespaceStatement::parse(pair)?);
            }
        }

        body.src_ref = pair.into();

        Ok(body)
    }
}

impl SrcReferrer for NamespaceBody {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Eval for NamespaceBody {
    type Output = ();

    fn eval(&self, context: &mut EvalContext) -> EvalResult<Self::Output> {
        for statement in &self.statements {
            statement.eval(context)?;
        }

        Ok(())
    }
}
