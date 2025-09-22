// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad literal syntax elements

mod number_literal;
mod units;

use microcad_core::*;
pub use number_literal::*;
pub use units::*;

use crate::{src_ref::*, syntax::*, ty::*, value::Value};

/// Literal of any kind.
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    /// Integer literal
    Integer(Refer<i64>),
    /// Number literal
    Number(NumberLiteral),
    /// Boolean literal
    Bool(Refer<bool>),
    Direction(Refer<Direction>),
    Alignment(Refer<Alignment>),
}

impl Literal {
    /// Return value of literal.
    pub fn value(&self) -> Value {
        match self {
            Self::Integer(value) => Value::Integer(*value.clone()),
            Self::Number(value) => value.value(),
            Self::Bool(value) => Value::Bool(*value.clone()),
            Self::Direction(value) => Value::Direction(**value),
            Self::Alignment(value) => Value::Alignment(**value),
        }
    }
}

impl SrcReferrer for Literal {
    fn src_ref(&self) -> SrcRef {
        match self {
            Literal::Number(n) => n.src_ref(),
            Literal::Integer(i) => i.src_ref(),
            Literal::Bool(b) => b.src_ref(),
            Literal::Direction(d) => d.src_ref(),
            Literal::Alignment(a) => a.src_ref(),
        }
    }
}

impl crate::ty::Ty for Literal {
    fn ty(&self) -> Type {
        match self {
            Literal::Integer(_) => Type::Integer,
            Literal::Number(n) => n.ty(),
            Literal::Bool(_) => Type::Bool,
            Literal::Direction(_) => Type::Direction,
            Literal::Alignment(_) => Type::Alignment,
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Literal::Integer(i) => write!(f, "{i}"),
            Literal::Number(n) => write!(f, "{n}"),
            Literal::Bool(b) => write!(f, "{b}"),
            Literal::Direction(d) => write!(f, "{d}"),
            Literal::Alignment(a) => write!(f, "{a}"),
        }
    }
}

impl From<Literal> for Value {
    fn from(literal: Literal) -> Self {
        literal.value()
    }
}

impl TreeDisplay for Literal {
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result {
        write!(f, "{:depth$}Literal: {self}", "")
    }
}
