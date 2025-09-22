// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Number literal syntax element

use crate::{parse::*, src_ref::*, syntax::*, ty::*, value::*};
use derive_more::Deref;
use microcad_core::Alignment;
use std::str::FromStr;

/// Number literal.
#[derive(Debug, Clone, PartialEq, Deref)]
pub struct AlignmentLiteral {
    #[deref]
    alignment: Alignment,
    src_ref: SrcRef,
}

impl AlignmentLiteral {
    /// Return value for number literal
    pub fn value(&self) -> Value {
        match &self.ty() {
            Type::Alignment => Value::Alignment(self.alignment),
            _ => unreachable!(),
        }
    }
}

impl crate::ty::Ty for AlignmentLiteral {
    fn ty(&self) -> Type {
        Type::Alignment
    }
}

impl SrcReferrer for AlignmentLiteral {
    fn src_ref(&self) -> literal::SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for AlignmentLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.alignment)
    }
}

impl From<AlignmentLiteral> for Value {
    fn from(literal: AlignmentLiteral) -> Self {
        literal.value()
    }
}

impl FromStr for AlignmentLiteral {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FIX" => Self,
        }
    }
}
