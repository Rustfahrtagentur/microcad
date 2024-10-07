// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Nested item list parser entity

use microcad_render::tree;

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};

/// Nested item list
#[derive(Clone, Debug)]
pub struct Nested(Vec<NestedItem>);

impl Parse for Nested {
    fn parse(pair: Pair) -> ParseResult<Self> {
        assert!(pair.as_rule() == Rule::nested || pair.as_rule() == Rule::expression_no_semicolon);

        Ok(Self(
            pair.inner()
                .filter(|pair| {
                    [Rule::qualified_name, Rule::call, Rule::node_body].contains(&pair.as_rule())
                })
                .map(NestedItem::parse)
                .collect::<ParseResult<_>>()?,
        ))
    }
}

impl SrcReferrer for Nested {
    fn src_ref(&self) -> expression::SrcRef {
        SrcRef::from_vec(&self.0)
    }
}

impl Eval for Nested {
    type Output = Value;

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        let mut values = Vec::new();
        for (index, item) in self.0.iter().enumerate() {
            match item {
                NestedItem::Call(call) => match call.eval(context)? {
                    Some(value) => {
                        values.push(value);
                    }
                    None => {
                        if index != 0 {
                            return Err(EvalError::CannotNestFunctionCall);
                        } else {
                            return Ok(Value::Scalar(Refer::new(0.0, call.src_ref())));
                            // TODO: This is a hack. Return a Option::None here
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
                NestedItem::NodeBody(body) => {
                    values.push(Value::Node(
                        context.descend_node(tree::group(), |context| body.eval(context))?,
                    ));
                }
            }
        }

        assert!(!values.is_empty());

        if values.len() == 1 {
            return Ok(values[0].clone());
        }

        let nodes = values
            .iter()
            .filter_map(|v| match v {
                Value::Node(node) => Some(node.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();

        // Finally, nest all nodes
        for node_window in nodes.windows(2) {
            node_window[0].append(node_window[1].clone());
        }

        Ok(Value::Node(nodes.first().unwrap().clone()))
    }
}

impl std::fmt::Display for Nested {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
