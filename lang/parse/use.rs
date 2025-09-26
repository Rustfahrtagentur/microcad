// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::*};

impl Parse for UseDeclaration {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::use_declaration);

        let mut inner = pair.inner();
        let first = inner.next().expect("Expected use declaration element");

        match first.as_rule() {
            Rule::qualified_name => Ok(Self::Use(QualifiedName::parse(first)?)),
            Rule::use_all => {
                let inner = first.inner().next().expect("Expected qualified name");
                Ok(Self::UseAll(QualifiedName::parse(inner)?))
            }
            Rule::use_alias => {
                let mut inner = first.inner();
                let name = QualifiedName::parse(inner.next().expect("Expected qualified name"))?;
                let alias = Identifier::parse(inner.next().expect("Expected identifier"))?;
                Ok(Self::UseAlias(name, alias))
            }
            _ => unreachable!("Invalid use declaration"),
        }
    }
}

impl Parse for UseStatement {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::use_statement);

        Ok(Self {
            visibility: crate::find_rule!(pair, visibility)?,
            decl: crate::find_rule_exact!(pair, use_declaration)?,
            src_ref: pair.into(),
        })
    }
}

impl Parse for Visibility {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::visibility);

        let s = pair.as_str();
        match s {
            "pub" => Ok(Self::Public),
            "" => Ok(Self::Private),
            _ => unreachable!("Invalid visibility"),
        }
    }
}
