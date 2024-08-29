use super::FunctionStatement;
use crate::{language::*, parser::*, with_pair_ok};

#[derive(Clone, Debug, Default)]
pub struct FunctionBody(pub Vec<FunctionStatement>);

impl Parse for FunctionBody {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        Parser::ensure_rule(&pair, Rule::function_body);

        let mut body = Vec::new();
        let p = pair.clone();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::function_statement => {
                    body.push(FunctionStatement::parse(pair)?.value);
                }
                Rule::expression => {
                    body.push(FunctionStatement::Return(Box::new(
                        Expression::parse(pair)?.value,
                    )));
                }
                rule => unreachable!("Unexpected token in function body: {:?}", rule),
            }
        }

        with_pair_ok!(Self(body), p)
    }
}
