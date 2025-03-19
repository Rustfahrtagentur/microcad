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
    Body(Body),
}

impl SrcReferrer for NestedItem {
    fn src_ref(&self) -> SrcRef {
        match self {
            Self::Call(c) => c.src_ref(),
            Self::QualifiedName(qn) => qn.src_ref(),
            Self::Body(nb) => nb.src_ref(),
        }
    }
}

impl Parse for NestedItem {
    fn parse(pair: Pair) -> ParseResult<Self> {
        match pair.clone().as_rule() {
            Rule::call => Ok(Self::Call(Call::parse(pair.clone())?)),
            Rule::qualified_name => Ok(Self::QualifiedName(QualifiedName::parse(pair.clone())?)),
            Rule::body => Ok(Self::Body(Body::parse(pair.clone())?)),
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
            Self::Body(body) => write!(f, "{}", body),
        }
    }
}

impl Syntax for NestedItem {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}NestedItem:", "")?;
        match self {
            Self::Call(call) => call.print_syntax(f, depth + 1),
            Self::QualifiedName(qualified_name) => qualified_name.print_syntax(f, depth + 1),
            Self::Body(body) => body.print_syntax(f, depth + 1),
        }
    }
}
