// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Expression statement syntax elements

use crate::{src_ref::*, syntax::*};

/// An assignment statement, e.g. `#[aux] s = sphere(3.0mm);`.
#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    attribute_list: AttributeList,
    expression: Expression,
    src_ref: SrcRef,
}

impl SrcReferrer for ExpressionStatement {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl PrintSyntax for ExpressionStatement {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}Expression '{}'", "", self.expression)
    }
}

impl std::fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.attribute_list.is_empty() {
            writeln!(f, "{}", self.attribute_list)?;
        }
        writeln!(f, "{};", self.expression)
    }
}

use crate::parser::*;

impl Parse for ExpressionStatement {
    fn parse(pair: Pair) -> crate::parse::ParseResult<Self> {
        Ok(Self {
            attribute_list: pair.find(Rule::attribute_list).unwrap_or_default(),
            expression: pair
                .find(Rule::expression)
                .or(pair.find(Rule::expression_no_semicolon))
                .expect("Expression"),
            src_ref: pair.into(),
        })
    }
}

use crate::eval::*;
use crate::value::*;

impl Eval for ExpressionStatement {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        self.expression
            .eval_with_attribute_list(&self.attribute_list, context)
    }
}
