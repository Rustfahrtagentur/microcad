// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Nested item parser entity

use crate::{parse::*, parser::*, src_ref::*};

/// Nested item
#[derive(Clone, Debug)]
pub enum NestedItem {
    /// Call
    Call(Call),
    /// Qualified Name
    QualifiedName(QualifiedName),
    /// Module body
    NodeBody(NodeBody),
}

impl Eval for NestedItem {
    type Output = CallResult;

    fn eval(&self, context: &mut EvalContext) -> EvalResult<Self::Output> {
        match &self {
            NestedItem::Call(call) => call.eval(context),
            NestedItem::QualifiedName(qualified_name) => match qualified_name.eval(context)? {
                Symbol::Value(_, Value::Node(node)) => {
                    Ok(CallResult::Nodes(vec![node.make_deep_copy()]))
                }
                Symbol::Value(_, value) => Ok(CallResult::Value(
                    value.clone_with_src_ref(qualified_name.src_ref()),
                )),
                symbol => {
                    context.error_with_stack_trace(self, EvalError::CannotNestSymbol(symbol))?;
                    Ok(CallResult::None)
                }
            },
            NestedItem::NodeBody(body) => Ok(CallResult::Nodes(vec![body.eval(context)?])),
        }
    }
}

impl SrcReferrer for NestedItem {
    fn src_ref(&self) -> SrcRef {
        match self {
            Self::Call(c) => c.src_ref(),
            Self::QualifiedName(qn) => qn.src_ref(),
            Self::NodeBody(nb) => nb.src_ref(),
        }
    }
}

impl Parse for NestedItem {
    fn parse(pair: Pair) -> ParseResult<Self> {
        match pair.clone().as_rule() {
            Rule::call => Ok(Self::Call(Call::parse(pair.clone())?)),
            Rule::qualified_name => Ok(Self::QualifiedName(QualifiedName::parse(pair.clone())?)),
            Rule::node_body => Ok(Self::NodeBody(NodeBody::parse(pair.clone())?)),
            rule => unreachable!(
                "NestedItem::parse expected call or qualified name, found {:?}",
                rule
            ),
        }
    }
}

impl std::fmt::Display for NestedItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Call(call) => write!(f, "{}", call),
            Self::QualifiedName(qualified_name) => write!(f, "{}", qualified_name),
            Self::NodeBody(body) => write!(f, "{}", body),
        }
    }
}
