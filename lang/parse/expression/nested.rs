// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Nested item list parser entity

use crate::{eval::*, parse::*, parser::*, src_ref::*};

/// Nested item list, e.g. an expression like `foo bar() {}`
#[derive(Clone, Debug)]
pub struct Nested(Refer<Vec<NestedItem>>);

impl Nested {
    /// Returns an identifier if the nested item is a single qualified name
    pub fn single_identifier(&self) -> Option<Identifier> {
        match self.0.first() {
            Some(NestedItem::QualifiedName(name)) => match name.as_slice() {
                [single_id] => Some(single_id.clone()),
                _ => None,
            },
            _ => None,
        }
    }
}

impl Parse for Nested {
    fn parse(pair: Pair) -> ParseResult<Self> {
        assert!(pair.as_rule() == Rule::nested || pair.as_rule() == Rule::expression_no_semicolon);

        Ok(Self(Refer::new(
            pair.inner()
                .filter(|pair| {
                    [Rule::qualified_name, Rule::call, Rule::node_body].contains(&pair.as_rule())
                })
                .map(NestedItem::parse)
                .collect::<ParseResult<_>>()?,
            pair.src_ref(),
        )))
    }
}

impl SrcReferrer for Nested {
    fn src_ref(&self) -> expression::SrcRef {
        self.0.src_ref()
    }
}

impl Eval for Nested {
    type Output = Option<Value>;

    fn eval(&self, context: &mut EvalContext) -> EvalResult<Self::Output> {
        let mut nodes = Vec::new();

        for item in self.0.iter() {
            match item.eval(context)? {
                CallResult::Nodes(n) => nodes.push(n),
                CallResult::None => {
                    if nodes.is_empty() && self.0.len() == 1 {
                        return Ok(None);
                    } else {
                        context.error_with_stack_trace(
                            self,
                            EvalError::CannotNestItem(item.clone()),
                        )?;
                    }
                }
                CallResult::Value(value) => {
                    if nodes.is_empty() && self.0.len() == 1 {
                        return Ok(Some(value));
                    } else {
                        context.error_with_stack_trace(
                            self,
                            EvalError::CannotNestItem(item.clone()),
                        )?;
                    }
                }
            }
        }

        if nodes.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Value::Node(crate::objects::nest_nodes(nodes))))
        }
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
