use strum::IntoStaticStr;

use super::identifier::*;
use crate::{eval::*, parser::*, with_pair_ok};

#[derive(Clone, Debug)]
pub struct UseAlias(pub QualifiedName, pub Identifier);

impl std::fmt::Display for UseAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "use {:?} as {:?}", self.0, self.1)
    }
}

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

impl Parse for UseAlias {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut inner = pair.clone().into_inner();
        with_pair_ok!(
            UseAlias(
                QualifiedName::parse(inner.next().unwrap())?.value().clone(),
                Identifier::parse(inner.next().unwrap())?.value().clone(),
            ),
            pair
        )
    }
}

impl Parse for UseStatement {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut inner = pair.clone().into_inner();
        let first = inner.next().unwrap();
        let second = inner.next();
        let names = Parser::vec(first.clone(), QualifiedName::parse)?
            .value()
            .clone();
        match (first.as_rule(), second) {
            (Rule::qualified_name_list, Some(second))
                if second.as_rule() == Rule::qualified_name =>
            {
                with_pair_ok!(
                    UseStatement::UseFrom(names, QualifiedName::parse(second)?.value().clone(),),
                    pair
                )
            }
            (Rule::qualified_name_list, None) => {
                with_pair_ok!(UseStatement::Use(names), pair)
            }
            (Rule::qualified_name_all, Some(second))
                if second.as_rule() == Rule::qualified_name_list =>
            {
                with_pair_ok!(
                    UseStatement::UseAll(
                        Parser::vec(second, QualifiedName::parse)?.value().clone()
                    ),
                    pair
                )
            }
            (Rule::use_alias, _) => {
                with_pair_ok!(
                    UseStatement::UseAlias(UseAlias::parse(first)?.value().clone()),
                    pair
                )
            }
            _ => Err(ParseError::InvalidUseStatement),
        }
    }
}

impl Eval for UseStatement {
    type Output = ();

    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error> {
        match self {
            UseStatement::UseAll(names) => {
                for name in names {
                    let symbols = name.eval(context)?;
                    for symbol in symbols {
                        match symbol {
                            Symbol::ModuleDefinition(module_definition) => {
                                module_definition
                                    .body
                                    .symbols
                                    .iter()
                                    .cloned()
                                    .for_each(|symbol| context.add_symbol(symbol));
                            }
                            _ => {
                                return Err(Error::ExpectedModule(name.clone()));
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
