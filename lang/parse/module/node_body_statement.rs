// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Node body statement parser entity

use crate::{eval::*, parse::*, parser::*, src_ref::*};

/// Module initialization statement
#[derive(Clone, Debug)]
pub enum NodeBodyStatement {
    /// Node marker, e.g. @children
    NodeMarker(NodeMarker),
    /// Use statement, e.g. `use std::math::sin`
    Use(UseStatement),
    /// Expression, e.g. `a + b`
    Expression(Expression),
    /// Assignment, e.g. `a = 1`
    Assignment(Assignment),
}

impl SrcReferrer for NodeBodyStatement {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        match self {
            Self::NodeMarker(marker) => marker.src_ref(),
            Self::Use(us) => us.src_ref(),
            Self::Expression(expression) => expression.src_ref(),
            Self::Assignment(assignment) => assignment.src_ref(),
        }
    }
}

impl Parse for NodeBodyStatement {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let first = pair.inner().next().expect(INTERNAL_PARSE_ERROR);
        Ok(match first.as_rule() {
            Rule::use_statement => NodeBodyStatement::Use(UseStatement::parse(first)?),
            Rule::expression | Rule::expression_no_semicolon => {
                NodeBodyStatement::Expression(Expression::parse(first)?)
            }
            Rule::assignment => NodeBodyStatement::Assignment(Assignment::parse(first)?),
            Rule::node_marker => NodeBodyStatement::NodeMarker(NodeMarker::parse(first)?),
            rule => return Err(ParseError::GrammarRuleError(format!("{rule:?}"))),
        })
    }
}

impl Eval for NodeBodyStatement {
    type Output = Option<Value>;

    fn eval(&self, context: &mut EvalContext) -> EvalResult<Self::Output> {
        match self {
            Self::NodeMarker(marker) => Ok(marker.eval(context)?.map(Value::Node)),
            Self::Use(use_statement) => {
                use_statement.eval(context)?;
                Ok(None)
            }
            Self::Expression(expression) => Ok(Some(expression.eval(context)?)),
            Self::Assignment(assignment) => {
                assignment.eval(context)?;
                Ok(None)
            }
        }
    }
}

impl std::fmt::Display for NodeBodyStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NodeMarker(marker) => write!(f, "{marker}"),
            Self::Assignment(assignment) => write!(f, "{assignment}"),
            Self::Expression(expression) => write!(f, "{expression}"),
            Self::Use(use_statement) => write!(f, "{use_statement}"),
        }
    }
}
