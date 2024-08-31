use crate::{eval::*, parse::*, parser::*};

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
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::module_statement);
        let first = pair.clone().into_inner().next().unwrap();
        Ok(match first.as_rule() {
            Rule::use_statement => ModuleStatement::Use(UseStatement::parse(first)?),
            Rule::expression => ModuleStatement::Expression(Expression::parse(first)?),
            Rule::assignment => ModuleStatement::Assignment(Assignment::parse(first)?),
            Rule::module_for_statement => ModuleStatement::For(ForStatement::parse(first)?),
            Rule::module_definition | Rule::namespace_definition => {
                ModuleStatement::ModuleDefinition(std::rc::Rc::new(ModuleDefinition::parse(first)?))
            }
            Rule::module_init_definition => ModuleStatement::ModuleInitDefinition(
                std::rc::Rc::new(ModuleInitDefinition::parse(first)?),
            ),
            Rule::function_definition => ModuleStatement::FunctionDefinition(std::rc::Rc::new(
                FunctionDefinition::parse(first)?,
            )),
            rule => unreachable!(
                "Unexpected module statement, got {:?} {:?}",
                rule,
                first.clone()
            ),
        })
    }
}

impl Eval for ModuleStatement {
    type Output = ();

    fn eval(&self, context: &mut Context) -> std::result::Result<Self::Output, EvalError> {
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
