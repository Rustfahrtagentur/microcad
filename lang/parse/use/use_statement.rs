use crate::{eval::*, parse::*, parser::*, src_ref::*};
use strum::IntoStaticStr;

#[derive(Clone, Debug, IntoStaticStr)]
pub enum UseStatement {
    /// Import symbols given as qualified names: `use a, b`
    Use(Vec<QualifiedName>),
    /// Import specific symbol from a module: `use a,b from c`
    UseFrom(Vec<QualifiedName>, QualifiedName),
    /// Import all symbols from a module: `use * from a, b`
    UseAll(Vec<QualifiedName>),
    /// Import as alias: `use a as b`
    UseAlias(UseAlias),
}

impl SrcReferer for UseStatement {
    fn src_ref(&self) -> SrcRef {
        match self {
            Self::Use(u) => SrcRef::from_vec(u),
            _ => todo!(),
        }
    }
}

impl std::fmt::Display for UseStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UseStatement::Use(names) => write!(f, "use {names:?}"),
            UseStatement::UseFrom(names, from) => write!(f, "use {names:?} from {from:?}"),
            UseStatement::UseAll(names) => write!(f, "use * from {names:?}"),
            UseStatement::UseAlias(alias) => write!(f, "{}", alias),
        }
    }
}

impl Parse for UseStatement {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let mut inner = pair.clone().into_inner();
        let first = inner.next().unwrap();
        let second = inner.next();
        let names = Parser::vec(first.clone(), QualifiedName::parse)?;
        match (first.as_rule(), second) {
            (Rule::qualified_name_list, Some(second))
                if second.as_rule() == Rule::qualified_name =>
            {
                Ok(UseStatement::UseFrom(names, QualifiedName::parse(second)?))
            }
            (Rule::qualified_name_list, None) => Ok(UseStatement::Use(names)),
            (Rule::qualified_name_all, Some(second))
                if second.as_rule() == Rule::qualified_name_list =>
            {
                Ok(UseStatement::UseAll(Parser::vec(
                    second,
                    QualifiedName::parse,
                )?))
            }
            (Rule::use_alias, _) => Ok(UseStatement::UseAlias(UseAlias::parse(first)?)),
            _ => Err(ParseError::InvalidUseStatement),
        }
    }
}

impl Eval for UseStatement {
    type Output = ();

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        match self {
            UseStatement::UseAll(names) => {
                for name in names {
                    let symbols = name.eval(context)?;
                    for symbol in symbols {
                        match symbol {
                            Symbol::ModuleDefinition(module_definition) => {
                                let symbols = &module_definition.body.symbols;
                                for symbol in symbols.iter() {
                                    context.add_symbol(symbol.clone());
                                }
                            }
                            _ => {
                                return Err(EvalError::ExpectedModule(
                                    name.id().expect("nameless module"),
                                ));
                            }
                        }
                    }
                }
                Ok(())
            }
            UseStatement::Use(names) => {
                for name in names {
                    let symbols = name.eval(context)?;
                    for symbol in symbols {
                        context.add_symbol(symbol.clone());
                    }
                }
                Ok(())
            }
            statement => {
                let s: &'static str = statement.into();
                unimplemented!(" {s}")
            }
        }
    }
}
