// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad format expression syntax elements

mod format_expression;
mod format_spec;

pub use format_expression::*;
pub use format_spec::*;

use crate::{src_ref::*, syntax::*};

/// Format string item
#[derive(Clone, Debug)]
pub enum FormatStringInner {
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
pub struct FormatString(pub Refer<Vec<FormatStringInner>>);

impl FormatString {
    /// Insert a string to this module
    pub fn push_string(&mut self, s: String, src_ref: SrcRef) {
        self.0
            .push(FormatStringInner::String(Refer::new(s, src_ref)));
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
        self.0.src_ref.clone()
    }
}

impl std::fmt::Display for FormatString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, r#"""#)?;
        for elem in &*self.0 {
            match elem {
                FormatStringInner::String(s) => write!(f, "{}", s.value)?,
                FormatStringInner::FormatExpression(expr) => write!(f, "{}", expr)?,
            }
        }
        write!(f, r#"""#)?;
        Ok(())
    }
}

impl PrintSyntax for FormatString {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}FormatString:", "")?;
        self.0.iter().try_for_each(|fs| match fs {
            FormatStringInner::String(s) => writeln!(f, "{:depth$}String: \"{}\"", "", s.value),
            FormatStringInner::FormatExpression(e) => e.print_syntax(f, depth + 1),
        })
    }
}
