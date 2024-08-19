use super::module::ModuleStatement;
use crate::{eval::*, parser::*, with_pair_ok};
use microcad_render::tree::{self, Node};

#[derive(Clone, Debug)]
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
    type Output = Node;

    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error> {
        let node = tree::root();
        context.set_current_node(node.clone());
        for statement in &self.body {
            statement.eval(context)?;
        }
        Ok(node)
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
