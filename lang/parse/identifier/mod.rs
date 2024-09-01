mod identifier_list;
mod qualified_name;

pub use identifier_list::*;
pub use qualified_name::*;

use crate::{eval::Sym, parser::*, src_ref::*};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier(microcad_core::Id, SrcRef);

impl SrcReferrer for Identifier {
    fn src_ref(&self) -> SrcRef {
        self.1.clone()
    }
}

impl std::hash::Hash for Identifier {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.0.hash(hasher)
    }
}

impl Sym for Identifier {
    fn id(&self) -> Option<microcad_core::Id> {
        Some(self.0.clone())
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Self(value.into(), SrcRef(None))
    }
}

impl<'a> From<&'a Identifier> for &'a str {
    fn from(value: &'a Identifier) -> Self {
        &value.0
    }
}

impl Parse for Identifier {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::identifier);
        Ok(Self(pair.as_str().into(), pair.into()))
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
