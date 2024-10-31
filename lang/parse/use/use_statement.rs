// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Use statement parser entity

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};
use strum::IntoStaticStr;

/// Use statement
#[derive(Clone, Debug, IntoStaticStr)]
pub enum UseDeclaration {
    /// Import symbols given as qualified names: `use a, b`
    Use(QualifiedName, SrcRef),
    /// Import all symbols from a module: `use std::*`
    UseAll(QualifiedName, SrcRef),
    /// Import as alias: `use a as b`
    UseAlias(QualifiedName, Identifier, SrcRef),
}

impl SrcReferrer for UseDeclaration {
    fn src_ref(&self) -> SrcRef {
        match self {
            Self::Use(_, src_ref) => src_ref.clone(),
            Self::UseAll(_, src_ref) => src_ref.clone(),
            Self::UseAlias(_, _, src_ref) => src_ref.clone(),
        }
    }
}

impl std::fmt::Display for UseDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UseDeclaration::Use(name, _) => write!(f, "{name}"),
            UseDeclaration::UseAll(name, _) => write!(f, "{name}::*"),
            UseDeclaration::UseAlias(name, alias, _) => write!(f, "{name} as {alias}"),
        }
    }
}

impl Parse for UseDeclaration {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::use_declaration);

        let mut inner = pair.inner();
        let first = inner.next().unwrap();

        match first.as_rule() {
            Rule::use_all => {
                let inner = first.inner().next().unwrap();
                Ok(Self::UseAll(
                    QualifiedName::parse(inner)?,
                    first.clone().into(),
                ))
            }
            Rule::qualified_name => {
                Ok(Self::Use(QualifiedName::parse(first)?, pair.clone().into()))
            }
            Rule::use_alias => {
                let mut inner = first.inner();
                let name = QualifiedName::parse(inner.next().unwrap())?;
                let alias = Identifier::parse(inner.next().unwrap())?;
                Ok(Self::UseAlias(name, alias, pair.clone().into()))
            }
            _ => unreachable!("Invalid use declaration"),
        }
    }
}

impl Eval for UseDeclaration {
    type Output = ();

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        match self {
            Self::UseAll(name, _) => {
                match name.eval(context)? {
                    Symbol::Namespace(namespace_definition) => {
                        let symbols = &namespace_definition.body.symbols;
                        for (_, symbol) in symbols.iter() {
                            context.add(symbol.as_ref().clone());
                        }
                    }
                    symbol => {
                        use crate::diag::PushDiag;
                        use anyhow::anyhow;
                        context.error(self, anyhow!("Expected namespace definition, got {symbol}"));
                    }
                }

                Ok(())
            }
            Self::Use(name, _) => {
                let symbol = name.eval(context)?;
                context.add(symbol);
                Ok(())
            }
            Self::UseAlias(name, alias, _) => {
                let symbol = name.eval(context)?;
                context.add_alias(symbol, alias.id().expect("nameless alias"));
                Ok(())
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct UseStatement(Vec<UseDeclaration>, SrcRef);

impl SrcReferrer for UseStatement {
    fn src_ref(&self) -> SrcRef {
        self.1.clone()
    }
}

impl std::fmt::Display for UseStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "use ")?;
        for (i, decl) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{decl}")?;
        }
        Ok(())
    }
}

impl Parse for UseStatement {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::use_statement);

        let mut decls = Vec::new();
        for pair in pair.inner() {
            decls.push(UseDeclaration::parse(pair)?);
        }

        Ok(Self(decls, pair.into()))
    }
}

impl Eval for UseStatement {
    type Output = ();

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        for decl in &self.0 {
            decl.eval(context)?;
        }
        Ok(())
    }
}
