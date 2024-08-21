use crate::{
    eval::{Context, Error, Eval, Symbols},
    language::{
        assignment::Assignment, expression::Expression, function::FunctionDefinition,
        r#use::UseStatement,
    },
    parser::{Pair, Parse, ParseResult, Parser, Rule},
    with_pair_ok,
};

use super::{module_definition::ModuleDefinition, ForStatement, ModuleInitDefinition};

#[derive(Clone, Debug, strum::IntoStaticStr)]
pub enum ModuleStatement {
    Use(UseStatement),
    Expression(Expression),
    For(ForStatement),
    Assignment(Assignment),
    ModuleDefinition(std::rc::Rc<ModuleDefinition>),
    FunctionDefinition(std::rc::Rc<FunctionDefinition>),
    ModuleInitDefinition(std::rc::Rc<ModuleInitDefinition>),
}

impl Parse for ModuleStatement {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        Parser::ensure_rule(&pair, Rule::module_statement);
        let first = pair.clone().into_inner().next().unwrap();
        with_pair_ok!(
            match first.as_rule() {
                Rule::use_statement => {
                    ModuleStatement::Use(UseStatement::parse(first)?.value().clone())
                }
                Rule::expression => {
                    ModuleStatement::Expression(Expression::parse(first)?.value().clone())
                }
                Rule::assignment => {
                    ModuleStatement::Assignment(Assignment::parse(first)?.value().clone())
                }
                Rule::module_for_statement => {
                    ModuleStatement::For(ForStatement::parse(first)?.value().clone())
                }
                Rule::module_definition | Rule::namespace_definition =>
                    ModuleStatement::ModuleDefinition(std::rc::Rc::new(
                        ModuleDefinition::parse(first)?.value().clone(),
                    )),
                Rule::module_init_definition => ModuleStatement::ModuleInitDefinition(
                    std::rc::Rc::new(ModuleInitDefinition::parse(first)?.value().clone(),)
                ),
                Rule::function_definition => ModuleStatement::FunctionDefinition(std::rc::Rc::new(
                    FunctionDefinition::parse(first)?.value().clone(),
                )),
                rule => unreachable!(
                    "Unexpected module statement, got {:?} {:?}",
                    rule,
                    first.clone()
                ),
            },
            pair
        )
    }
}

impl Eval for ModuleStatement {
    type Output = ();

    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error> {
        match self {
            ModuleStatement::Use(use_statement) => {
                use_statement.eval(context)?;
            }
            ModuleStatement::Expression(expr) => {
                expr.eval(context)?;
            }
            ModuleStatement::Assignment(assignment) => {
                assignment.eval(context)?;
            }
            ModuleStatement::FunctionDefinition(function_definition) => {
                context.add_function(function_definition.clone());
            }
            ModuleStatement::ModuleDefinition(module_definition) => {
                context.add_module(module_definition.clone());
            }
            statement => {
                let s: &'static str = statement.into();
                unimplemented!("ModuleStatement::{s}")
            }
        }

        Ok(())
    }
}

impl std::fmt::Display for ModuleStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ModuleStatement::Use(use_statement) => write!(f, "{use_statement}"),
            ModuleStatement::Expression(expression) => write!(f, "{expression}"),
            ModuleStatement::Assignment(assignment) => write!(f, "{assignment}"),
            ModuleStatement::For(for_statement) => write!(f, "{for_statement}"),
            ModuleStatement::ModuleDefinition(module_definition) => {
                write!(f, "{}", module_definition.name)
            }
            ModuleStatement::FunctionDefinition(function_definition) => {
                write!(f, "{}", function_definition.name)
            }
            ModuleStatement::ModuleInitDefinition(_) => write!(f, "module init"),
        }
    }
}
