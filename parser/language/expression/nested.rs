use crate::{eval::*, language::*, parser::*, with_pair_ok};

#[derive(Clone, Debug)]
pub struct Nested(Vec<NestedItem>);

impl Parse for Nested {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut vec = Vec::new();
        for pair in pair.clone().into_inner().filter(|pair| {
            [Rule::qualified_name, Rule::call, Rule::module_body].contains(&pair.as_rule())
        }) {
            vec.push(NestedItem::parse(pair)?.value().clone());
        }

        with_pair_ok!(Nested(vec), pair)
    }
}

impl Eval for Nested {
    type Output = Value;

    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error> {
        let root = context.current_node();

        let mut values = Vec::new();
        for (index, item) in self.0.iter().enumerate() {
            match item {
                NestedItem::Call(call) => match call.eval(context)? {
                    Some(value) => values.push(value),
                    None => {
                        if index != 0 {
                            return Err(Error::CannotNestFunctionCall);
                        } else {
                            return Ok(Value::Scalar(0.0)); // @todo This is a hack. Return a Option::None here
                        }
                    }
                },
                NestedItem::QualifiedName(qualified_name) => {
                    let symbols = qualified_name.eval(context)?;

                    for symbol in symbols {
                        if let Symbol::Value(_, v) = symbol {
                            values.push(v.clone()); // Find first value only. @todo Back propagation of values
                            break;
                        }
                    }
                }
                NestedItem::ModuleBody(body) => {
                    let new_node = body.eval(context)?;
                    new_node.detach();
                    values.push(Value::Node(new_node));
                }
            }
        }

        assert!(!values.is_empty());

        if values.len() == 1 {
            return Ok(values[0].clone());
        }

        // Finally, nest all nodes
        for value in values {
            match value {
                Value::Node(node) => {
                    node.detach();
                    let nested = context.append_node(node);
                    context.set_current_node(nested);
                }
                _ => {
                    return Err(Error::CannotNestFunctionCall);
                }
            }
        }

        context.set_current_node(root.clone());

        Ok(Value::Node(root.clone()))
    }
}

impl std::fmt::Display for Nested {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            " {}",
            self.0
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}
