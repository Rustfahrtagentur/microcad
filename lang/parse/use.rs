use crate::{parse::*, parser::*, syntax::*};

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

impl Parse for UseStatement {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::use_statement);

        let mut visibility = Visibility::default();
        let mut decls = Vec::new();

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

        Ok(Self {
            visibility,
            decls,
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
            "private" => Ok(Self::Private),
            _ => unreachable!("Invalid visibility"),
        }
    }
}
