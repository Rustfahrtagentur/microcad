use crate::with_pair_ok;

use super::{CallArgumentList, Identifier, Pair, Parse, ParseResult};

#[derive(Clone, Debug, Default)]
pub struct MethodCall {
    pub name: Identifier,
    pub argument_list: CallArgumentList,
}

impl Parse for MethodCall {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut inner = pair.clone().into_inner();
        with_pair_ok!(
            MethodCall {
                name: Identifier::parse(inner.next().unwrap())?.value().clone(),
                argument_list: if let Some(pair) = inner.next() {
                    CallArgumentList::parse(pair)?.value().clone()
                } else {
                    CallArgumentList::default()
                },
            },
            pair
        )
    }
}

impl std::fmt::Display for MethodCall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({:?})", self.name, self.argument_list)
    }
}
