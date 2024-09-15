// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µCAD attribute parser entity

use crate::{errors::*, parse::*, parser::*, src_ref::*};

/// Attribute entity
#[derive(Clone, Debug)]
pub struct Attribute {
    /// Name of the attribute
    pub name: QualifiedName,
    /// Arguments of the attribute
    pub arguments: Option<CallArgumentList>,
    /// Source code reference
    src_ref: SrcRef,
}

impl SrcReferrer for Attribute {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.src_ref.clone()
    }
}

impl Parse for Attribute {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut inner = pair.inner();
        let name = QualifiedName::parse(inner.next().unwrap())?;
        Ok(Attribute {
            name,
            arguments: match inner.next() {
                Some(pair) => Some(CallArgumentList::parse(pair.clone())?),
                _ => None,
            },
            src_ref: pair.clone().into(),
        })
    }
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.arguments {
            Some(arguments) => write!(f, "{}({:?})", self.name, arguments),
            None => write!(f, "{}", self.name),
        }
    }
}
