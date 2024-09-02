//! Nested item parser entity

use crate::{errors::*, parse::*, parser::*, src_ref::*};

/// Nested item
#[derive(Clone, Debug)]
pub enum NestedItem {
    /// Call
    Call(Call),
    /// Qualified Name
    QualifiedName(QualifiedName),
    /// Module body
    ModuleBody(ModuleBody),
}

impl SrcReferrer for NestedItem {
    fn src_ref(&self) -> SrcRef {
        match self {
            Self::Call(c) => c.src_ref(),
            Self::QualifiedName(qn) => qn.src_ref(),
            Self::ModuleBody(mb) => mb.src_ref(),
        }
    }
}

impl Parse for NestedItem {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        match pair.clone().as_rule() {
            Rule::call => Ok(Self::Call(Call::parse(pair.clone())?)),
            Rule::qualified_name => Ok(Self::QualifiedName(QualifiedName::parse(pair.clone())?)),
            Rule::module_body => Ok(Self::ModuleBody(ModuleBody::parse(pair.clone())?)),
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
            Self::ModuleBody(body) => write!(f, "{}", body),
        }
    }
}
