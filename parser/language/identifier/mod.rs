mod identifier_list;
mod qualified_name;

use std::ops::Deref;

pub use identifier_list::*;
pub use qualified_name::*;

use crate::{parser::*, with_pair_ok};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Identifier(microcad_core::Identifier);

impl Deref for Identifier {
    type Target = microcad_core::Identifier;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::str::FromStr for Identifier {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Self(value.into())
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
        self.0 == *other
    }
}

pub fn join_identifiers(identifiers: &[Identifier], separator: &str) -> String {
    identifiers
        .iter()
        .map(|ident| ident.to_string())
        .collect::<Vec<_>>()
        .join(separator)
}
