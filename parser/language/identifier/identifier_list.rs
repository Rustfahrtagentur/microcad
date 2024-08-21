use crate::{language::*, parser::*, with_pair_ok};
use thiserror::Error;

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

impl From<Vec<Identifier>> for IdentifierList {
    fn from(value: Vec<Identifier>) -> Self {
        Self(value)
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
