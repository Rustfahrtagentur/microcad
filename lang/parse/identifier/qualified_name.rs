// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, src_ref::*};

/// A qualifier name consists of a . separated list of identifiers
/// e.g. `a.b.c`
#[derive(Debug, Default, Clone, PartialEq)]
pub struct QualifiedName(pub Vec<Identifier>);

impl SrcReferrer for QualifiedName {
    fn src_ref(&self) -> SrcRef {
        SrcRef::from_vec(&self.0)
    }
}

impl Sym for QualifiedName {
    fn id(&self) -> Option<Id> {
        // TODO: how to convert qualified name into one single id?
        self.last().map(|i| i.id().clone())
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
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(Self(
            pair.inner()
                .map(|pair| Identifier::parse(pair).expect("Expected identifier"))
                .collect(),
        ))
    }
}

impl std::fmt::Display for QualifiedName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", join_identifiers(&self.0, "::"))
    }
}

impl QualifiedName {
    /// Visit all symbols in the qualified name recursively, starting from the root
    pub fn visit_symbols(
        &self,
        context: &EvalContext,
        functor: &mut dyn FnMut(&Symbol, usize),
    ) -> EvalResult<()> {
        self._visit_symbols(None, 0, context, functor)
    }

    /// Visit all symbols in the qualified name recursively
    fn _visit_symbols(
        &self,
        root: Option<std::rc::Rc<Symbol>>,
        index: usize,
        context: &EvalContext,
        functor: &mut dyn FnMut(&Symbol, usize),
    ) -> EvalResult<()> {
        if index >= self.0.len() {
            return Ok(());
        }

        let new_symbol = match (&root, &self.0[index].id()) {
            (Some(root), id) => root.fetch_symbols(id),
            (None, id) => context.fetch(id),
        };

        if let Some(symbol) = new_symbol {
            functor(&symbol, index);
            self._visit_symbols(Some(symbol.clone()), index + 1, context, functor)?;
        }

        Ok(())
    }

    /// Get all symbols for the qualified name
    pub fn fetch_symbols(&self, context: &mut EvalContext) -> EvalResult<Vec<Symbol>> {
        let mut symbols = Vec::new();
        self.visit_symbols(context, &mut |symbol, depth| {
            // Only take symbols that match the full qualified name
            if depth == self.0.len() - 1 {
                symbols.push(symbol.clone());
            }
        })?;

        if symbols.is_empty() {
            context.error_with_stack_trace(
                self,
                EvalError::SymbolNotFound(self.id().unwrap_or_default()),
            )?;
        }
        Ok(symbols)
    }

    /// Get the symbol for the qualified name
    ///
    /// If there are multiple symbols with the same name, an error is returned
    pub fn fetch_symbol(&self, context: &mut EvalContext) -> EvalResult<Option<Symbol>> {
        let symbols = self.fetch_symbols(context)?;
        if symbols.len() > 1 {
            context.error_with_stack_trace(
                self,
                EvalError::AmbiguousSymbol(symbols.first().expect(INTERNAL_PARSE_ERROR).clone()),
            )?;
            // TODO Output all symbols
        }
        Ok(symbols.into_iter().next())
    }
}

impl Eval for QualifiedName {
    type Output = Symbol;

    fn eval(&self, context: &mut EvalContext) -> EvalResult<Self::Output> {
        if let Some(symbol) = self.fetch_symbol(context)? {
            Ok(symbol)
        } else {
            Ok(Symbol::default())
        }
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
