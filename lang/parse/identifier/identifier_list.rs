use crate::{errors::*, parse::*, parser::*};

/// A list of identifiers
///
/// Used e.g. for multiple variable declarations.
/// Cannot contain duplicates.
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
    pub fn push(&mut self, ident: Identifier) -> Result<(), ParseError> {
        if self.contains(&ident) {
            Err(ParseError::DuplicateIdentifier(ident))
        } else {
            self.0.push(ident);
            Ok(())
        }
    }
    pub fn extend(&mut self, other: IdentifierList) -> Result<(), ParseError> {
        for ident in other {
            self.push(ident)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for IdentifierList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", join_identifiers(&self.0, ", "))
    }
}

impl Parse for IdentifierList {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let mut vec = Vec::new();
        for pair in pair.clone().into_inner() {
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
