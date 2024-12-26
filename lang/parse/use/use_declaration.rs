// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Use statement parser entity

use crate::{eval::*, parse::*, parser::*, src_ref::*, sym::*};
use strum::IntoStaticStr;

/// Use declaration
///
/// A use declaration is an element of a use statement.
/// It can be a single symbol, all symbols from a module, or an alias.
///
/// ```ucad
/// use std::print;
/// use std::*;
/// use std::print as p;
/// ```
///
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
        let first = inner.next().expect("Expected use declaration element");

        match first.as_rule() {
            Rule::qualified_name => {
                Ok(Self::Use(QualifiedName::parse(first)?, pair.clone().into()))
            }
            Rule::use_all => {
                let inner = first.inner().next().expect("Expected qualified name");
                Ok(Self::UseAll(
                    QualifiedName::parse(inner)?,
                    first.clone().into(),
                ))
            }
            Rule::use_alias => {
                let mut inner = first.inner();
                let name = QualifiedName::parse(inner.next().expect("Expected qualified name"))?;
                let alias = Identifier::parse(inner.next().expect("Expected identfier"))?;
                Ok(Self::UseAlias(name, alias, pair.clone().into()))
            }
            _ => unreachable!("Invalid use declaration"),
        }
    }
}

impl Eval for UseDeclaration {
    type Output = SymbolTable;

    fn eval(&self, context: &mut Context) -> EvalResult<Self::Output> {
        match self {
            Self::UseAll(name, _) => match name.eval(context)? {
                Symbol::Namespace(namespace_definition) => {
                    Ok(namespace_definition.body.symbols.clone())
                }
                symbol => {
                    context
                        .error_with_stack_trace(self, EvalError::NamespaceSymbolExpected(symbol))?;
                    Ok(SymbolTable::default())
                }
            },
            Self::Use(name, _) => {
                let mut symbols = SymbolTable::default();

                let symbol = name.eval(context)?;
                if matches!(symbol, Symbol::Invalid) {
                    context.error_with_stack_trace(self, EvalError::CannotUse(symbol))?;
                } else {
                    symbols.add(symbol);
                }
                Ok(symbols)
            }
            Self::UseAlias(name, alias, _) => {
                let mut symbols = SymbolTable::default();
                symbols.add_alias(name.eval(context)?, alias.id().clone());
                Ok(symbols)
            }
        }
    }
}
