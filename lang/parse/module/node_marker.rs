// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Node marker parser entity

use crate::{eval::*, objects::*, parse::*, parser::*, src_ref::*};

/// Node marker, e.g. `@children`
#[derive(Clone, Debug)]
pub struct NodeMarker {
    /// Marker name, e.g. `children`
    pub name: Identifier,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl NodeMarker {
    /// Returns true if the marker is a children marker
    pub fn is_children_marker(&self) -> bool {
        &self.name == "children"
    }
}

impl SrcReferrer for NodeMarker {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.src_ref.clone()
    }
}

impl Parse for NodeMarker {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::node_marker);
        Ok(Self {
            name: Identifier::parse(pair.inner().next().expect(INTERNAL_PARSE_ERROR))?,
            src_ref: pair.src_ref(),
        })
    }
}

impl Eval for NodeMarker {
    type Output = Option<ObjectNode>;

    fn eval(&self, context: &mut EvalContext) -> EvalResult<Self::Output> {
        match self.name.to_string().as_str() {
            "children" => Ok(Some(crate::objects::ObjectNode::new(
                crate::objects::ObjectNodeInner::ChildrenNodeMarker,
            ))),
            _ => {
                context.error_with_stack_trace(
                    self,
                    EvalError::InvalidNodeMarker(self.name.clone()),
                )?;
                Ok(None)
            }
        }
    }
}

impl std::fmt::Display for NodeMarker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.name)
    }
}
