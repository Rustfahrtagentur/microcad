use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};


#[derive(Debug, Clone)]
pub enum NamespaceStatement {
    Use(UseStatement),
    ModuleDefinition(std::rc::Rc<ModuleDefinition>),
    NamespaceDefinition(std::rc::Rc<NamespaceDefinition>),
    FunctionDefinition(std::rc::Rc<FunctionDefinition>),
    Assignment(Assignment),
}

impl SrcReferrer for NamespaceStatement {
    fn src_ref(&self) -> SrcRef {
        match self {
            Self::Use(us) => us.src_ref(),
            Self::ModuleDefinition(md) => md.src_ref(),
            Self::NamespaceDefinition(nd) => nd.src_ref(),
            Self::FunctionDefinition(fd) => fd.src_ref(),
            Self::Assignment(a) => a.src_ref(),
        }
    }
}

impl Parse for NamespaceStatement {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::namespace_statement);
        let first = pair.clone().into_inner().next().unwrap();
        Ok(match first.as_rule() {
            Rule::use_statement => Self::Use(UseStatement::parse(first)?),
            Rule::module_definition => Self::ModuleDefinition(std::rc::Rc::<ModuleDefinition>::parse(first)?),
            Rule::namespace_definition => Self::NamespaceDefinition(std::rc::Rc::<NamespaceDefinition>::parse(first)?),
            Rule::function_definition => Self::FunctionDefinition(std::rc::Rc::<FunctionDefinition>::parse(first)?),
            Rule::assignment => Self::Assignment(Assignment::parse(first)?),
            rule => unreachable!(
                "Unexpected namespace statement, got {:?} {:?}",
                rule,
                first.clone()
            ),
        }) 
    }
}

impl Eval for NamespaceStatement {
    type Output = ();

        fn eval(&self, context: &mut Context) -> std::result::Result<Self::Output, EvalError> {
            match self {
                Self::Use(use_statement) => {
                    use_statement.eval(context)?;
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
                Self::NamespaceDefinition(namespace_definition) => {
                    context.add_namespace(namespace_definition.clone());
                }
            }
    
            Ok(())
        }
    }
