use super::module::*;
use crate::eval::*;
use crate::{parser::*, with_pair_ok};

#[derive(Clone)]
pub struct Document {
    body: Vec<ModuleStatement>,
}

impl Parse for Document {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut body = Vec::new();

        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::module_statement => {
                    body.push(ModuleStatement::parse(pair)?.value().clone());
                }
                Rule::EOI => break,
                _ => {}
            }
        }

        with_pair_ok!(Document { body }, pair)
    }
}

impl Eval for Document {
    type Output = ();

    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error> {
        for statement in &self.body {
            statement.eval(context)?;
        }
        Ok(())
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
