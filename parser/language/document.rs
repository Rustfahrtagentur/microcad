use super::module::*;
use crate::{parser::*, with_pair_ok};

#[derive(Clone)]
pub struct Document {
    body: Vec<ModuleStatement>,
}

impl Parse for Document {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut body = Vec::new();
        let p = pair.clone();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::module_statement => {
                    body.push(ModuleStatement::parse(pair)?.value().clone());
                }
                Rule::EOI => break,
                _ => {}
            }
        }

        with_pair_ok!(Document { body }, p)
    }
}

#[test]
fn document() {
    let document = Parser::parse_rule_or_panic::<Document>(
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
