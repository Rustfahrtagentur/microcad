use crate::parser::{Pair, Parse, ParseError};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
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

impl Parse for Identifier {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        Ok(Self(pair.as_str().into()))
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct IdentifierList(Vec<Identifier>);

impl Parse for IdentifierList {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut vec = Vec::new();
        for pair in pair.into_inner() {
            vec.push(Identifier(pair.as_str().into()));
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
            .join(".");
        write!(f, "{}", s)
    }
}

impl From<&str> for QualifiedName {
    fn from(value: &str) -> Self {
        let mut name = Vec::new();
        for ident in value.split('.') {
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
            .join(".");
        s
    }
}
