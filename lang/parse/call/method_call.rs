use crate::{parse::*, parser::*, with_pair_ok};

#[derive(Clone, Debug)]
pub struct MethodCall {
    pub name: Identifier,
    pub argument_list: CallArgumentList,
}

impl Parse for MethodCall {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut inner = pair.clone().into_inner();
        with_pair_ok!(
            MethodCall {
                name: Identifier::parse(inner.next().unwrap())?.value,
                argument_list: if let Some(pair) = inner.next() {
                    CallArgumentList::parse(pair)?.value
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
