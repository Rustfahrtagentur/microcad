use super::FunctionStatement;
use crate::{parse::*, parser::*};

#[derive(Clone, Debug, Default)]
pub struct FunctionBody(pub Vec<FunctionStatement>);

impl Parse for FunctionBody {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::function_body);

        let mut body = Vec::new();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::function_statement => {
                    body.push(FunctionStatement::parse(pair)?);
                }
                Rule::expression => {
                    body.push(FunctionStatement::Return(Box::new(Expression::parse(
                        pair,
                    )?)));
                }
                rule => unreachable!("Unexpected token in function body: {:?}", rule),
            }
        }

        Ok(Self(body))
    }
}
