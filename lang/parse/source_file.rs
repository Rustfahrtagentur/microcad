use crate::{eval::*, parse::*, parser::*};
use microcad_render::tree;

#[derive(Clone, Debug)]
pub struct SourceFile {
    body: Vec<ModuleStatement>,
}

impl Parse for SourceFile {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let mut body = Vec::new();

        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::module_statement => {
                    body.push(ModuleStatement::parse(pair)?);
                }
                Rule::EOI => break,
                _ => {}
            }
        }

        Ok(SourceFile { body })
    }
}

impl Eval for SourceFile {
    type Output = tree::Node;

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
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
    let document = Parser::parse_rule_or_panic::<SourceFile>(
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
