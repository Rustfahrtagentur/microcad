// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Number literal syntax element

use crate::{parse::*, src_ref::*, syntax::*, ty::*, value::*};
use derive_more::Deref;
use microcad_core::Direction;
use std::str::FromStr;

/// Number literal.
#[derive(Debug, Clone, PartialEq, Deref)]
pub struct DirectionLiteral {
    #[deref]
    direction: Direction,
    src_ref: SrcRef,
}

impl DirectionLiteral {
    /// Return value for number literal
    pub fn value(&self) -> Value {
        match &self.ty() {
            Type::Direction => Value::Direction(self.direction.clone()),
            _ => unreachable!(),
        }
    }
}

impl crate::ty::Ty for DirectionLiteral {
    fn ty(&self) -> Type {
        Type::Direction
    }
}

impl SrcReferrer for DirectionLiteral {
    fn src_ref(&self) -> literal::SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for DirectionLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.direction)
    }
}

impl From<DirectionLiteral> for Value {
    fn from(literal: DirectionLiteral) -> Self {
        literal.value()
    }
}

impl FromStr for DirectionLiteral {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
