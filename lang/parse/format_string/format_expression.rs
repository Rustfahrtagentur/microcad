// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Format expression parser entity

use crate::{parse::*, parser::*, src_ref::*};

/// Format expression including format specification
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct FormatExpression {
    /// Format specifier
    pub spec: FormatSpec,
    /// Expression to format
    pub expression: Expression,
    /// Source code reference
    src_ref: SrcRef,
}

impl FormatExpression {
    /// Create new format expression
    pub fn new(spec: FormatSpec, expression: Expression) -> Self {
        Self {
            src_ref: SrcRef::merge(spec.src_ref(), expression.src_ref()),
            spec,
            expression,
        }
    }
}

impl Parse for FormatExpression {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(Self::new(
            pair.find(Rule::format_spec).unwrap_or_default(),
            pair.find(Rule::expression).expect("Missing expression"),
        ))
    }
}

impl SrcReferrer for FormatExpression {
    fn src_ref(&self) -> SrcRef {
        SrcRef::merge(&self.spec, &self.expression)
    }
}
