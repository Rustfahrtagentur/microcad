use crate::{
    language::identifier::{Identifier, QualifiedName},
    parser::{Pair, Parse, ParseResult},
    with_pair_ok,
};

#[derive(Clone, Debug)]
pub struct UseAlias(pub QualifiedName, pub Identifier);

impl std::fmt::Display for UseAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "use {:?} as {:?}", self.0, self.1)
    }
}

impl Parse for UseAlias {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut inner = pair.clone().into_inner();
        with_pair_ok!(
            UseAlias(
                QualifiedName::parse(inner.next().unwrap())?.value().clone(),
                Identifier::parse(inner.next().unwrap())?.value().clone(),
            ),
            pair
        )
    }
}
