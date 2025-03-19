// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Format expression parser entity

use crate::{parse::*, parser::*, src_ref::*};

/// Format expression including format specification
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct FormatExpression {
    /// Format specifier
    pub spec: Option<FormatSpec>,
    /// Expression to format
    pub expression: Expression,
    /// Source code reference
    src_ref: SrcRef,
}

impl FormatExpression {
    /// Create new format expression
    pub fn new(spec: Option<FormatSpec>, expression: Expression) -> Self {
        Self {
            src_ref: match &spec {
                Some(spec) => SrcRef::merge(spec.src_ref(), expression.src_ref()),
                None => expression.src_ref(),
            },
            spec,
            expression,
        }
    }
}

impl Parse for FormatExpression {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(Self::new(
            pair.find(Rule::format_spec),
            pair.find(Rule::expression).expect("Missing expression"),
        ))
    }
}

impl std::fmt::Display for FormatExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(spec) = &self.spec {
            write!(f, "{{{}:{}}}", spec, self.expression)
        } else {
            write!(f, "{{{}}}", self.expression)
        }
    }
}

impl SrcReferrer for FormatExpression {
    fn src_ref(&self) -> SrcRef {
        if let Some(spec) = &self.spec {
            SrcRef::merge(spec, &self.expression)
        } else {
            self.expression.src_ref()
        }
    }
}

impl Syntax for FormatExpression {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}FormatExpression:", "")?;
        if let Some(spec) = &self.spec {
            spec.print_syntax(f, depth + 1)?;
            self.expression.print_syntax(f, depth + 1)
        } else {
            self.expression.print_syntax(f, depth + 1)
        }
    }
}
