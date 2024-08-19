use super::{FunctionBody, FunctionDefinition};
use crate::{
    language::{assignment::Assignment, expression::Expression, use_statement::UseStatement},
    parser::{Pair, Parse, ParseResult, Parser, Rule},
    with_pair_ok,
};

#[derive(Clone, Debug)]
pub enum FunctionStatement {
    Assignment(Assignment),
    Use(UseStatement),
    FunctionDefinition(std::rc::Rc<FunctionDefinition>),
    Return(Box<Expression>),
    If {
        condition: Expression,
        if_body: FunctionBody,
        else_body: FunctionBody,
    },
}

impl Parse for FunctionStatement {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        Parser::ensure_rule(&pair, Rule::function_statement);

        let mut inner = pair.clone().into_inner();
        let first = inner.next().unwrap();
        let s = match first.as_rule() {
            Rule::assignment => Self::Assignment(Assignment::parse(first)?.value().clone()),
            Rule::use_statement => Self::Use(UseStatement::parse(first)?.value().clone()),
            Rule::function_definition => Self::FunctionDefinition(std::rc::Rc::new(
                FunctionDefinition::parse(first)?.value().clone(),
            )),
            Rule::function_return_statement => {
                Self::Return(Box::new(Expression::parse(first)?.value().clone()))
            }
            Rule::function_if_statement => {
                let mut pairs = first.into_inner();
                let condition = Expression::parse(pairs.next().unwrap())?.value().clone();
                let if_body = FunctionBody::parse(pairs.next().unwrap())?.value().clone();

                match pairs.next() {
                    None => Self::If {
                        condition,
                        if_body,
                        else_body: FunctionBody::default(),
                    },
                    Some(pair) => {
                        let else_body = FunctionBody::parse(pair)?.value().clone();
                        Self::If {
                            condition,
                            if_body,
                            else_body,
                        }
                    }
                }
            }
            rule => unreachable!("Unexpected token in function statement: {:?}", rule),
        };

        with_pair_ok!(s, pair)
    }
}
