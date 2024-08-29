use crate::{language::*, parser::*, with_pair_ok};

#[derive(Clone, Debug)]
pub enum ModuleInitStatement {
    Use(UseStatement),
    Expression(Expression),
    Assignment(Assignment),
    FunctionDefinition(FunctionDefinition),
}

impl Parse for ModuleInitStatement {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let first = pair.clone().into_inner().next().unwrap();
        with_pair_ok!(
            match first.as_rule() {
                Rule::use_statement => {
                    ModuleInitStatement::Use(UseStatement::parse(first)?.value)
                }
                Rule::expression => {
                    ModuleInitStatement::Expression(Expression::parse(first)?.value)
                }
                Rule::assignment => {
                    ModuleInitStatement::Assignment(Assignment::parse(first)?.value)
                }
                Rule::function_definition =>
                    ModuleInitStatement::FunctionDefinition(FunctionDefinition::parse(first)?.value,),
                _ => unreachable!(),
            },
            pair
        )
    }
}
