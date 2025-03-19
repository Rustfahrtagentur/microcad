// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Nested item list parser entity

use crate::{parse::*, parser::*, src_ref::*};

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
                    [Rule::qualified_name, Rule::call, Rule::body].contains(&pair.as_rule())
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

impl std::fmt::Display for Nested {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl PrintSyntax for Nested {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}Nested:", "")?;
        self.0
            .iter()
            .try_for_each(|ni| ni.print_syntax(f, depth + 1))
    }
}
