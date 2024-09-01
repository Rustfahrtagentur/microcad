use crate::{eval::*, parse::*, parser::*, src_ref::*};

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

impl SrcReferrer for ModuleStatement {
    fn src_ref(&self) -> SrcRef {
        match self {
            Self::Use(us) => us.src_ref(),
            Self::Expression(e) => e.src_ref(),
            Self::For(fs) => fs.src_ref(),
            Self::Assignment(a) => a.src_ref(),
            Self::ModuleDefinition(md) => md.src_ref(),
            Self::FunctionDefinition(fd) => fd.src_ref(),
            Self::ModuleInitDefinition(mid) => mid.src_ref(),
        }
    }
}

impl Parse for ModuleStatement {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::module_statement);
        let first = pair.clone().into_inner().next().unwrap();
        Ok(match first.as_rule() {
            Rule::use_statement => Self::Use(UseStatement::parse(first)?),
            Rule::expression => Self::Expression(Expression::parse(first)?),
            Rule::assignment => Self::Assignment(Assignment::parse(first)?),
            Rule::module_for_statement => Self::For(ForStatement::parse(first)?),
            Rule::module_definition | Rule::namespace_definition => {
                Self::ModuleDefinition(std::rc::Rc::new(ModuleDefinition::parse(first)?))
            }
            Rule::module_init_definition => {
                Self::ModuleInitDefinition(std::rc::Rc::new(ModuleInitDefinition::parse(first)?))
            }
            Rule::function_definition => {
                Self::FunctionDefinition(std::rc::Rc::new(FunctionDefinition::parse(first)?))
            }
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
            Self::Use(use_statement) => {
                use_statement.eval(context)?;
            }
            Self::Expression(expr) => {
                expr.eval(context)?;
            }
            Self::Assignment(assignment) => {
                assignment.eval(context)?;
            }
            Self::FunctionDefinition(function_definition) => {
                context.add_function(function_definition.clone());
            }
            Self::ModuleDefinition(module_definition) => {
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
            Self::Use(use_statement) => write!(f, "{use_statement}"),
            Self::Expression(expression) => write!(f, "{expression}"),
            Self::Assignment(assignment) => write!(f, "{assignment}"),
            Self::For(for_statement) => write!(f, "{for_statement}"),
            Self::ModuleDefinition(module_definition) => {
                write!(f, "{}", module_definition.name)
            }
            Self::FunctionDefinition(function_definition) => {
                write!(f, "{}", function_definition.name)
            }
            Self::ModuleInitDefinition(_) => write!(f, "module init"),
        }
    }
}
