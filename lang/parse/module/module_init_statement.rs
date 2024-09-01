use crate::{parse::*, parser::*, src_ref::SrcReferrer};

#[derive(Clone, Debug)]
pub enum ModuleInitStatement {
    Use(UseStatement),
    Expression(Expression),
    Assignment(Assignment),
    FunctionDefinition(FunctionDefinition),
}

impl SrcReferrer for ModuleInitStatement {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        match self {
            Self::Use(us) => us.src_ref(),
            Self::Expression(us) => us.src_ref(),
            Self::Assignment(us) => us.src_ref(),
            Self::FunctionDefinition(us) => us.src_ref(),
        }
    }
}

impl Parse for ModuleInitStatement {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let first = pair.clone().into_inner().next().unwrap();
        Ok(match first.as_rule() {
            Rule::use_statement => ModuleInitStatement::Use(UseStatement::parse(first)?),
            Rule::expression => ModuleInitStatement::Expression(Expression::parse(first)?),
            Rule::assignment => ModuleInitStatement::Assignment(Assignment::parse(first)?),
            Rule::function_definition => {
                ModuleInitStatement::FunctionDefinition(FunctionDefinition::parse(first)?)
            }
            _ => unreachable!(),
        })
    }
}
