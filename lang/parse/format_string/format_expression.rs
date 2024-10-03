// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Format expression parser entity

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};

/// Format expression including format specification
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct FormatExpression {
    /// Format specifier
    pub spec: FormatSpec,
    /// Expression to format
    pub expression: Box<Expression>,
    /// Source code reference
    src_ref: SrcRef,
}

impl FormatExpression {
    /// Create new format expression
    pub fn new(spec: FormatSpec, expression: Expression) -> Self {
        Self {
            src_ref: SrcRef::merge(spec.src_ref(), expression.src_ref()),
            spec,
            expression: Box::new(expression),
        }
    }
}

impl Parse for FormatExpression {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(Self::new(
            Parser::find(&pair, Rule::format_spec).unwrap_or_default(),
            Parser::find(&pair, Rule::expression).expect("Missing expression"),
        ))
    }
}

impl SrcReferrer for FormatExpression {
    fn src_ref(&self) -> SrcRef {
        SrcRef::merge(&self.spec, self.expression.as_ref())
    }
}

impl Eval for FormatExpression {
    type Output = Value;

    fn eval(&self, context: &mut Context) -> Result<Value> {
        Ok(Value::String(Refer::new(
            format!("{}", self.expression.eval(context)?),
            SrcRef(None),
        )))
    }
}

impl std::fmt::Display for FormatExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", self.expression)
    }
}
