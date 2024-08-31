use crate::{parse::*, parser::*};

#[derive(Clone, Debug)]
pub struct MethodCall {
    pub name: Identifier,
    pub argument_list: CallArgumentList,
}

impl Parse for MethodCall {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let mut inner = pair.clone().into_inner();

        Ok(MethodCall {
            name: Identifier::parse(inner.next().unwrap())?,
            argument_list: if let Some(pair) = inner.next() {
                CallArgumentList::parse(pair)?
            } else {
                CallArgumentList::default()
            },
        })
    }
}

impl std::fmt::Display for MethodCall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({:?})", self.name, self.argument_list)
    }
}
