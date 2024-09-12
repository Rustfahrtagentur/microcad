// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};

/// A qualifier name consists of a . separated list of identifiers
/// e.g. `a.b.c`
#[derive(Debug, Default, Clone, PartialEq)]
pub struct QualifiedName(Vec<Identifier>);

impl SrcReferrer for QualifiedName {
    fn src_ref(&self) -> SrcRef {
        SrcRef::from_vec(&self.0)
    }
}

impl Sym for QualifiedName {
    fn id(&self) -> Option<microcad_core::Id> {
        // TODO: how to convert qualified name into one single id?
        self.last().and_then(|i| i.id())
    }
}

impl std::ops::Deref for QualifiedName {
    type Target = Vec<Identifier>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for QualifiedName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Parse for QualifiedName {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        Ok(Self(
            pair.clone()
                .into_inner()
                .map(|pair| Identifier::parse(pair))
                .map(|ident| ident.unwrap())
                .collect(),
        ))
    }
}

impl std::fmt::Display for QualifiedName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", join_identifiers(&self.0, "::"))
    }
}

impl QualifiedName {
    /// Visit all symbols in the qualified name recursively, starting from the root
    pub fn visit_symbols(
        &self,
        context: &Context,
        functor: &mut dyn FnMut(&Symbol, usize),
    ) -> Result<()> {
        self._visit_symbols(None, 0, context, functor)
    }

    /// Visit all symbols in the qualified name recursively
    fn _visit_symbols(
        &self,
        root: Option<Symbol>,
        index: usize,
        context: &Context,
        functor: &mut dyn FnMut(&Symbol, usize),
    ) -> Result<()> {
        if index >= self.0.len() {
            return Ok(());
        }
        let ident = &self.0[index];

        let new_symbols = match (&root, ident.id()) {
            (Some(ref root), Some(id)) => root.fetch_symbols(&id),
            (None, Some(id)) => context.fetch_symbols(&id),
            _ => unreachable!("can't search unnamed symbol"),
        };

        for symbol in new_symbols {
            functor(symbol, index);
            self._visit_symbols(Some(symbol.clone()), index + 1, context, functor)?;
        }

        Ok(())
    }

    /// Get all symbols for the qualified name
    pub fn fetch_symbols(&self, context: &Context) -> Result<Vec<Symbol>> {
        let mut symbols = Vec::new();
        self.visit_symbols(context, &mut |symbol, depth| {
            // Only take symbols that match the full qualified name
            if depth == self.0.len() - 1 {
                symbols.push(symbol.clone());
            }
        })?;

        if symbols.is_empty() {
            return Err(EvalError::SymbolNotFound(
                self.id().expect("unnamed symbol not found"),
            ));
        }
        Ok(symbols)
    }
}

impl Eval for QualifiedName {
    type Output = Vec<Symbol>;

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        self.fetch_symbols(context)
    }
}

impl From<&str> for QualifiedName {
    fn from(value: &str) -> Self {
        Self(value.split("::").map(Identifier::from).collect())
    }
}

impl From<QualifiedName> for String {
    fn from(value: QualifiedName) -> Self {
        join_identifiers(&value.0, "::")
    }
}

