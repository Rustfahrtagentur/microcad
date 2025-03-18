// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad format expression parser entities

mod format_expression;
mod format_spec;

pub use format_expression::*;
pub use format_spec::*;

use crate::{parse::*, parser::*, src_ref::*};

/// Format string item
#[derive(Clone, Debug)]
enum FormatStringInner {
    /// String literal
    String(Refer<String>),
    /// Format expression
    FormatExpression(FormatExpression),
}

impl SrcReferrer for FormatStringInner {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        match self {
            FormatStringInner::String(s) => s.src_ref(),
            FormatStringInner::FormatExpression(e) => e.src_ref(),
        }
    }
}

/// Format string
#[derive(Default, Clone, Debug)]
pub struct FormatString(Vec<FormatStringInner>);

impl std::str::FromStr for FormatString {
    type Err = ParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Parser::parse_rule::<Self>(Rule::format_string, s, 0)
    }
}

impl FormatString {
    /// Insert a string to this module
    pub fn push_string(&mut self, s: String) {
        self.0.push(FormatStringInner::String(Refer::none(s)));
    }

    /// Insert a format expression to this module
    pub fn push_format_expr(&mut self, expr: FormatExpression) {
        self.0.push(FormatStringInner::FormatExpression(expr));
    }

    /// Return the number of sections (inserted elements)
    pub fn section_count(&self) -> usize {
        self.0.len()
    }
}

impl SrcReferrer for FormatString {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        SrcRef::from_vec(&self.0)
    }
}

impl std::fmt::Display for FormatString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, r#"""#)?;
        for elem in &self.0 {
            match elem {
                FormatStringInner::String(s) => write!(f, "{}", s.value)?,
                FormatStringInner::FormatExpression(expr) => write!(f, "{}", expr)?,
            }
        }
        write!(f, r#"""#)?;
        Ok(())
    }
}

impl Parse for FormatString {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut fs = Self::default();
        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::string_literal_inner => fs.push_string(pair.as_span().as_str().to_string()),
                Rule::format_expression => fs.push_format_expr(FormatExpression::parse(pair)?),
                _ => unreachable!(),
            }
        }

        Ok(fs)
    }
}
