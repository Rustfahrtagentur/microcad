use crate::{eval::*, language::*, parser::*, with_pair_ok};

/// A qualifier name consists of a . separated list of identifiers
/// e.g. `a.b.c`
#[derive(Debug, Default, Clone, PartialEq)]
pub struct QualifiedName(Vec<Identifier>);

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
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        with_pair_ok!(
            Self(
                pair.clone()
                    .into_inner()
                    .map(|pair| Identifier::parse(pair))
                    .map(|ident| ident.unwrap().value().clone())
                    .collect(),
            ),
            pair
        )
    }
}

impl std::fmt::Display for QualifiedName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .0
            .iter()
            .map(|ident| ident.0.clone())
            .collect::<Vec<_>>()
            .join("::");
        write!(f, "{}", s)
    }
}

impl QualifiedName {
    /// @brief Visit all symbols in the qualified name recursively, starting from the root
    pub fn visit_symbols(
        &self,
        context: &Context,
        functor: &mut dyn FnMut(&Symbol, usize),
    ) -> Result<(), Error> {
        self._visit_symbols(None, 0, context, functor)
    }

    /// @brief Visit all symbols in the qualified name recursively
    fn _visit_symbols(
        &self,
        root: Option<Symbol>,
        index: usize,
        context: &Context,
        functor: &mut dyn FnMut(&Symbol, usize),
    ) -> Result<(), Error> {
        if index >= self.0.len() {
            return Ok(());
        }
        let ident = &self.0[index];

        let new_symbols = match root {
            Some(ref root) => root.get_symbols(ident),
            None => context.find_symbols(ident),
        };

        for symbol in new_symbols {
            functor(symbol, index);
            self._visit_symbols(Some(symbol.clone()), index + 1, context, functor)?;
        }

        Ok(())
    }

    /// @brief Get all symbols for the qualified name
    pub fn get_symbols(&self, context: &Context) -> Result<Vec<Symbol>, Error> {
        let mut symbols = Vec::new();
        self.visit_symbols(context, &mut |symbol, depth| {
            // Only take symbols that match the full qualified name
            if depth == self.0.len() - 1 {
                symbols.push(symbol.clone());
            }
        })?;

        if symbols.is_empty() {
            return Err(Error::SymbolNotFound(self.clone()));
        }
        Ok(symbols)
    }
}

impl Eval for QualifiedName {
    type Output = Vec<Symbol>;

    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error> {
        self.get_symbols(context)
    }
}

impl From<&str> for QualifiedName {
    fn from(value: &str) -> Self {
        Self(value.split("::").map(Identifier::from).collect())
    }
}

impl From<QualifiedName> for String {
    fn from(value: QualifiedName) -> Self {
        value
            .0
            .iter()
            .map(|identifier| identifier.0.clone())
            .collect::<Vec<_>>()
            .join("::")
    }
}
