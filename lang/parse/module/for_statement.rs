use crate::{parse::*, parser::*};

#[derive(Clone, Debug)]
pub struct ForStatement {
    loop_var: Assignment,
    body: ModuleBody,
}

impl Parse for ForStatement {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::module_for_statement);

        let mut pairs = pair.into_inner();

        let loop_var = Assignment::parse(pairs.next().unwrap())?;
        let body = ModuleBody::parse(pairs.next().unwrap())?;

        Ok(ForStatement { loop_var, body })
    }
}

impl std::fmt::Display for ForStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "for {} {}", self.loop_var, self.body)
    }
}
