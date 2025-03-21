use crate::{parse::*, parser::*, syntax::*};

impl Parse for Body {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rules(&pair, &[Rule::body, Rule::body_else]);
        let mut body = Self::default();
        for pair in pair.inner() {
            if pair.as_rule() == Rule::statement {
                body.statements.push(Statement::parse(pair.clone())?)
            }
        }
        body.src_ref = pair.into();

        Ok(body)
    }
}
