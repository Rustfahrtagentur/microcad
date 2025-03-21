use crate::{parser::*, src_ref::*, *};

impl Parse for IdentifierList {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut vec = Vec::new();
        for pair in pair.inner() {
            if pair.as_rule() == Rule::identifier {
                vec.push(Identifier::parse(pair)?);
            }
        }
        Ok(Self(Refer::new(vec, pair.into())))
    }
}

impl Parse for Identifier {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::identifier);
        Ok(Self(Refer::new(pair.as_str().into(), pair.into())))
    }
}

impl Parse for QualifiedName {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(Self(
            pair.inner()
                .map(|pair| Identifier::parse(pair).expect("Expected identifier"))
                .collect(),
        ))
    }
}
