use crate::{eval::*, parser::*, with_pair_ok};
use thiserror::Error;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier(String);

pub enum Visibility {
    Private,
    Public,
}

impl Identifier {
    /// @brief Every identifier starting with '_' is private
    pub fn visibility(self) -> Visibility {
        if self.0.starts_with('_') {
            Visibility::Private
        } else {
            Visibility::Public
        }
    }
}

impl std::ops::Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::str::FromStr for Identifier {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<Identifier> for String {
    fn from(value: Identifier) -> Self {
        Self::from(&value.0)
    }
}

impl<'a> From<&'a Identifier> for &'a str {
    fn from(value: &'a Identifier) -> Self {
        &value.0
    }
}

impl Parse for Identifier {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        Parser::ensure_rule(&pair, Rule::identifier);
        with_pair_ok!(Self(pair.as_str().into()), pair)
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<str> for Identifier {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

#[derive(Debug, Error)]
pub enum IdentifierListError {
    #[error("Duplicate identifier: {0}")]
    DuplicateIdentifier(Identifier),
}

/// @brief A list of identifiers
/// @details Used e.g. for multiple variable declarations.
///          Cannot contain duplicates.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct IdentifierList(Vec<Identifier>);

impl IdentifierList {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl std::ops::Deref for IdentifierList {
    type Target = Vec<Identifier>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IdentifierList {
    pub fn push(&mut self, ident: Identifier) -> Result<(), IdentifierListError> {
        if self.contains(&ident) {
            Err(IdentifierListError::DuplicateIdentifier(ident))
        } else {
            self.0.push(ident);
            Ok(())
        }
    }
    pub fn extend(&mut self, other: IdentifierList) -> Result<(), IdentifierListError> {
        for ident in other {
            self.push(ident)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for IdentifierList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .iter()
            .map(|ident| ident.0.clone())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", s)
    }
}

impl Parse for IdentifierList {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut vec = Vec::new();
        for pair in pair.clone().into_inner() {
            if pair.as_rule() == Rule::identifier {
                vec.push(Identifier::parse(pair)?.value().clone());
            }
        }
        with_pair_ok!(Self(vec), pair)
    }
}

impl std::iter::IntoIterator for IdentifierList {
    type Item = Identifier;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

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

    /// @brief Visit all symbols in the qualified name recursively, starting from the root
    pub fn visit_symbols(
        &self,
        context: &Context,
        functor: &mut dyn FnMut(&Symbol, usize),
    ) -> Result<(), Error> {
        self._visit_symbols(None, 0, context, functor)
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
        let mut name = Vec::new();
        for ident in value.split("::") {
            name.push(Identifier(ident.to_string()));
        }
        Self(name)
    }
}

impl From<QualifiedName> for String {
    fn from(value: QualifiedName) -> Self {
        let s = value
            .0
            .iter()
            .map(|ident| ident.0.clone())
            .collect::<Vec<_>>()
            .join("::");
        s
    }
}
