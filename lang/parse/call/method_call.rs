use crate::{parse::*, parser::*, src_ref::*};

#[derive(Clone, Debug)]
pub struct MethodCall {
    pub name: Identifier,
    pub argument_list: CallArgumentList,
    src_ref: SrcRef,
}

impl SrcReferrer for MethodCall {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
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
            src_ref: pair.into(),
        })
    }
}

impl std::fmt::Display for MethodCall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({:?})", self.name, self.argument_list)
    }
}
