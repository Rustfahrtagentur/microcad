// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Nested item list parser entity

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
                        use crate::diag::PushDiag;
                        if index > 0 {
                            context
                                .error(self, anyhow::anyhow!("Cannot nest function call {item}"))?;
                        }
                        return Ok(Value::Invalid);
                    }
                },
                NestedItem::QualifiedName(qualified_name) => {
                    let symbol = qualified_name.eval(context)?;
                    match symbol {
                        Symbol::Value(_, ref v) => {
                            match v {
                                Value::Node(node) => {
                                    values.push(Value::Node(node.make_deep_copy()));
                                }
                                _ => {
                                    values.push(v.clone_with_src_ref(qualified_name.src_ref()));
                                }
                            }

                            break;
                        }
                        Symbol::Invalid => {
                            use crate::diag::PushDiag;
                            context.error(
                                self,
                                anyhow::anyhow!("Symbol not found: {}", qualified_name),
                            )?;
                            return Ok(Value::Invalid);
                        }
                        _ => {}
                    }
                }
                NestedItem::NodeBody(body) => {
                    values.push(Value::Node(body.eval(context)?));
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

        Ok(Value::Node(crate::objecttree::nest_nodes(nodes)))
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
