use crate::{
    language::assignment::Assignment,
    parser::{Pair, Parse, ParseResult, Parser, Rule},
    with_pair_ok,
};

use super::ModuleBody;

#[derive(Clone, Debug)]
pub struct ForStatement {
    loop_var: Assignment,
    body: ModuleBody,
}

impl Parse for ForStatement {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let p = pair.clone();
        Parser::ensure_rule(&pair, Rule::module_for_statement);

        let mut pairs = pair.into_inner();

        let loop_var = Assignment::parse(pairs.next().unwrap())?;
        let body = ModuleBody::parse(pairs.next().unwrap())?;

        with_pair_ok!(
            ForStatement {
                loop_var: loop_var.value().clone(),
                body: body.value().clone(),
            },
            p
        )
    }
}

impl std::fmt::Display for ForStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "for {} {}", self.loop_var, self.body)
    }
}