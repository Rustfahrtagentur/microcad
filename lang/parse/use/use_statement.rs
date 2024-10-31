// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Use statement parser entity

use std::ascii::escape_default;

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
    type Output = SymbolTable;

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        match self {
            Self::UseAll(name, _) => match name.eval(context)? {
                Symbol::Namespace(namespace_definition) => {
                    Ok(namespace_definition.body.symbols.clone())
                }
                symbol => {
                    use crate::diag::PushDiag;
                    use anyhow::anyhow;
                    context.error(self, anyhow!("Expected namespace definition, got {symbol}"))?;
                    Ok(SymbolTable::default())
                }
            },
            Self::Use(name, _) => {
                let mut symbols = SymbolTable::default();

                let symbol = name.eval(context)?;
                if let Symbol::None = symbol {
                } else {
                    symbols.add(symbol);
                }

                Ok(symbols)
            }
            Self::UseAlias(name, alias, _) => {
                let mut symbols = SymbolTable::default();
                symbols.add_alias(name.eval(context)?, alias.id().expect("nameless alias"));
                Ok(symbols)
            }
        }
    }
}

/// Use statement:
///
/// ```ucad
///
/// use std::*;
/// ```
#[derive(Clone, Debug)]
pub struct UseStatement(Visibility, Vec<UseDeclaration>, SrcRef);

impl SrcReferrer for UseStatement {
    fn src_ref(&self) -> SrcRef {
        self.2.clone()
    }
}

impl std::fmt::Display for UseStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.0 {
            Visibility::Private => write!(f, "use ")?,
            Visibility::Public => write!(f, "pub use ")?,
        }
        for (i, decl) in self.1.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{decl}")?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub enum Visibility {
    /// Private visibility
    #[default]
    Private,
    /// Public visibility
    Public,
}

impl Parse for Visibility {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::visibility);

        let s = pair.as_str();
        match s {
            "pub" => Ok(Self::Public),
            _ => unreachable!("Invalid visibility"),
        }
    }
}

impl Parse for UseStatement {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::use_statement);

        let mut decls = Vec::new();
        let mut visibility = Visibility::default();

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::use_declaration => {
                    decls.push(UseDeclaration::parse(pair)?);
                }
                Rule::visibility => {
                    visibility = Visibility::parse(pair)?;
                }
                _ => unreachable!("Invalid use declaration"),
            }
        }

        Ok(Self(visibility, decls, pair.into()))
    }
}

impl Eval for UseStatement {
    type Output = Option<SymbolTable>;

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        // Return a symbol table if the use statement is public
        match self.0 {
            Visibility::Public => {
                let mut symbols = SymbolTable::default();
                for decl in &self.1 {
                    let mut symbol_table = decl.eval(context)?;
                    for (name, symbol) in symbol_table.iter() {
                        use crate::eval::Symbols;
                        context.add_alias(symbol.as_ref().clone(), name.clone());
                    }

                    symbols.merge(&mut symbol_table);
                }
                Ok(Some(symbols))
            }
            Visibility::Private => {
                for decl in &self.1 {
                    let symbol_table = decl.eval(context)?;
                    for (name, symbol) in symbol_table.iter() {
                        use crate::eval::Symbols;
                        context.add_alias(symbol.as_ref().clone(), name.clone());
                    }
                }
                Ok(None)
            }
        }
    }
}
