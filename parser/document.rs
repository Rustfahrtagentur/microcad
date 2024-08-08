use crate::{
    module::ModuleStatement,
    parser::{Pair, Parse, ParseError, Rule},
};

pub struct Document {
    body: Vec<ModuleStatement>,
}

impl Parse for Document {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut body = Vec::new();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::module_statement => {
                    body.push(ModuleStatement::parse(pair)?);
                }
                Rule::EOI => break,
                _ => {}
            }
        }

        Ok(Document { body })
    }
}

#[cfg(test)]
mod tests {

    use crate::parser::Rule;

    use super::*;

    #[test]
    fn document() {
        let document = crate::parser::Parser::parse_rule_or_panic::<Document>(
            Rule::document,
            r#"use std::io::println;
            module foo(r: scalar) {
                info("Hello, world, {r}!");
            }
            foo(20.0);
            "#,
        );

        assert_eq!(document.body.len(), 3);
    }
}
