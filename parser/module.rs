// Resolve a qualified name to a type or value.

use crate::{
    syntaxtree::{UseAlias, UseStatement},
    CsglParser, FunctionArgument, Identifier, Parse, ParseError, QualifiedName, Rule,
};

pub struct Constructor {
    arguments: Vec<FunctionArgument>,
}

pub struct Module {
    name: Identifier,
    constructor: Vec<FunctionArgument>,
}

impl crate::Parse for UseAlias {
    fn parse(pair: pest::iterators::Pair<Rule>) -> Result<Self, crate::ParseError> {
        let mut pairs = pair.into_inner();
        let first = pairs.next().unwrap();
        let second = pairs.next().unwrap();

        Ok(UseAlias(
            CsglParser::qualified_name(first)?,
            CsglParser::identifier(second)?,
        ))
    }
}

impl crate::Parse for UseStatement {
    fn parse(pair: pest::iterators::Pair<Rule>) -> Result<Self, crate::ParseError> {
        let mut pairs = pair.into_inner();

        let first = pairs.next().unwrap();
        let second = pairs.next();

        match first.as_rule() {
            Rule::qualified_name_list => {
                let qualified_name_list = CsglParser::qualified_name_list(first.into_inner())?;
                if let Some(second) = second {
                    if second.as_rule() == Rule::qualified_name {
                        return Ok(UseStatement::UseFrom(
                            qualified_name_list,
                            CsglParser::qualified_name(second)?,
                        ));
                    } else {
                        unreachable!();
                    }
                } else {
                    return Ok(UseStatement::Use(qualified_name_list));
                }
            }
            Rule::qualified_name_all => {
                if let Some(second) = second {
                    if second.as_rule() == Rule::qualified_name_list {
                        return Ok(UseStatement::UseAll(CsglParser::qualified_name_list(
                            second.into_inner(),
                        )?));
                    } else {
                        unreachable!();
                    }
                }
            }
            Rule::use_alias => {
                if let Some(second) = second {
                    return Ok(UseStatement::UseAliasFrom(
                        UseAlias::parse(first)?,
                        CsglParser::qualified_name(second)?,
                    ));
                } else {
                    return Ok(UseStatement::UseAlias(UseAlias::parse(first)?));
                }
            }

            _ => unreachable!(),
        }

        Err(ParseError::InvalidUseStatement)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_qualified_name() {
        let used_modules = vec!["shape2d", "math"];

        let qualified_names: Vec<QualifiedName> = vec!["shape2d.circle".into(), "math.PI".into()];
    }
}
