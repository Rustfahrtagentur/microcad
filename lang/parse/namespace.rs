use crate::{Rc, parse::*, parser::*, syntax::*};

impl Parse for Rc<NamespaceDefinition> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut pairs = pair.inner();
        Ok(Rc::new(NamespaceDefinition {
            name: Identifier::parse(pairs.next().expect("Identifier expected"))?,
            body: Body::parse(pairs.next().expect("NamespaceBody expected"))?,
            src_ref: pair.clone().into(),
        }))
    }
}
