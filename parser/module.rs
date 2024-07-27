// Resolve a qualified name to a type or value.

use crate::parser::*;
use crate::syntax_tree::{UseAlias, UseStatement};

pub struct Constructor {
    arguments: Vec<FunctionArgument>,
}

pub struct Module {
    name: Identifier,
    constructor: Vec<FunctionArgument>,
}

impl Parse for UseAlias {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut pairs = pair.into_inner();
        let first = pairs.next().unwrap();
        let second = pairs.next().unwrap();

        Ok(UseAlias(
            Parser::qualified_name(first)?,
            Parser::identifier(second)?,
        ))
    }
}

impl Parse for UseStatement {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut pairs = pair.into_inner();

        let first = pairs.next().unwrap();
        let second = pairs.next();

        match first.as_rule() {
            Rule::qualified_name_list => {
                let qualified_name_list = Parser::qualified_name_list(first.into_inner())?;
                if let Some(second) = second {
                    if second.as_rule() == Rule::qualified_name {
                        return Ok(UseStatement::UseFrom(
                            qualified_name_list,
                            Parser::qualified_name(second)?,
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
                        return Ok(UseStatement::UseAll(Parser::qualified_name_list(
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
                        Parser::qualified_name(second)?,
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
        let used_modules = vec!["primitives", "math"];

        let qualified_names: Vec<QualifiedName> =
            vec!["primitives.circle".into(), "math.PI".into()];
    }
}
