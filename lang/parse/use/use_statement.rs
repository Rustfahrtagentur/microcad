// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Use statement parser entity

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};
use strum::IntoStaticStr;

/// Use statement
#[derive(Clone, Debug, IntoStaticStr)]
pub enum UseStatement {
    /// Import symbols given as qualified names: `use a, b`
    Use(Refer<Vec<QualifiedName>>),
    /// Import specific symbol from a module: `use a,b from c`
    UseFrom(Vec<QualifiedName>, QualifiedName, SrcRef),
    /// Import all symbols from a module: `use * from a, b`
    UseAll(Refer<Vec<QualifiedName>>),
    /// Import as alias: `use a as b`
    UseAlias(Refer<UseAlias>),
}

impl SrcReferrer for UseStatement {
    fn src_ref(&self) -> SrcRef {
        match self {
            Self::Use(u) => u.src_ref(),
            Self::UseAll(ua) => ua.src_ref(),
            Self::UseAlias(ua) => ua.src_ref(),
            Self::UseFrom(_, _, src_ref) => src_ref.clone(),
        }
    }
}

impl std::fmt::Display for UseStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UseStatement::Use(names) => write!(f, "use {names:?}"),
            UseStatement::UseFrom(names, from, _) => write!(f, "use {names:?} from {from:?}"),
            UseStatement::UseAll(names) => write!(f, "use * from {names:?}"),
            UseStatement::UseAlias(alias) => write!(f, "{alias}"),
        }
    }
}

impl Parse for UseStatement {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut inner = pair.inner();
        let first = inner.next().unwrap();
        let second = inner.next();
        let names = Parser::vec(first.clone(), QualifiedName::parse)?;
        match (first.as_rule(), second) {
            (Rule::qualified_name_list, Some(second))
                if second.as_rule() == Rule::qualified_name =>
            {
                Ok(UseStatement::UseFrom(
                    names,
                    QualifiedName::parse(second)?,
                    pair.clone().into(),
                ))
            }
            (Rule::qualified_name_list, None) => {
                Ok(UseStatement::Use(Refer::new(names, pair.clone().into())))
            }
            (Rule::qualified_name_all, Some(second))
                if second.as_rule() == Rule::qualified_name_list =>
            {
                Ok(UseStatement::UseAll(Refer::new(
                    Parser::vec(second, QualifiedName::parse)?,
                    pair.clone().into(),
                )))
            }
            (Rule::use_alias, _) => Ok(UseStatement::UseAlias(Refer::new(
                UseAlias::parse(first)?,
                pair.clone().into(),
            ))),
            _ => Err(ParseError::InvalidUseStatement),
        }
    }
}

impl Eval for UseStatement {
    type Output = ();

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        match self {
            UseStatement::UseAll(names) => {
                for name in &names.value {
                    let symbols = name.eval(context)?;
                    for symbol in symbols {
                        match symbol {
                            Symbol::Namespace(namespace_definition) => {
                                let symbols = &namespace_definition.body.symbols;
                                for (_, symbol) in symbols.iter() {
                                    context.add(symbol.as_ref().clone());
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
                for name in &names.value {
                    let symbols = name.eval(context)?;
                    for symbol in symbols {
                        context.add(symbol.clone());
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
