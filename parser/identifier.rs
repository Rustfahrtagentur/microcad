use crate::{
    eval::{Context, Eval},
    parser::{Pair, Parse, ParseError, Parser},
    Rule,
};

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
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        Parser::ensure_rule(&pair, Rule::identifier);
        Ok(Self(pair.as_str().into()))
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
    pub fn get(&self, index: usize) -> Option<&Identifier> {
        self.0.get(index)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<Identifier> {
        self.0.iter()
    }

    pub fn push(&mut self, ident: Identifier) -> Result<(), IdentifierListError> {
        if self.contains(&ident) {
            Err(IdentifierListError::DuplicateIdentifier(ident))
        } else {
            self.0.push(ident);
            Ok(())
        }
    }

    pub fn contains(&self, ident: &Identifier) -> bool {
        self.0.contains(ident)
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
            .0
            .iter()
            .map(|ident| ident.0.clone())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", s)
    }
}

impl Parse for IdentifierList {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut vec = Vec::new();
        for pair in pair.into_inner() {
            if pair.as_rule() == Rule::identifier {
                vec.push(Identifier::parse(pair)?);
            }
        }
        Ok(Self(vec))
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

impl QualifiedName {
    pub fn last(&self) -> &Identifier {
        self.0.last().unwrap()
    }

    pub fn push(&mut self, ident: Identifier) {
        self.0.push(ident);
    }
}

impl Parse for QualifiedName {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        Ok(Self(
            pair.into_inner()
                .map(|pair| Identifier::parse(pair))
                .map(|ident| ident.unwrap())
                .collect(),
        ))
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

impl Eval for QualifiedName {
    type Output = crate::eval::Symbol;

    fn eval(&self, context: &mut Context) -> Result<Self::Output, crate::eval::Error> {
        let mut symbol = None;

        for (i, ident) in self.0.iter().enumerate() {
            if i == 0 {
                match context.get_symbol(ident.into()) {
                    Some(s) => {
                        symbol = Some(s.clone());
                    }
                    _ => return Err(crate::eval::Error::SymbolNotFound(ident.clone())),
                }
            } else {
                symbol = match symbol {
                    Some(crate::eval::Symbol::ModuleDefinition(module)) => module.get_symbol(ident),
                    _ => return Err(crate::eval::Error::SymbolNotFound(ident.clone())),
                }
            }
        }

        Ok(symbol.unwrap())
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
