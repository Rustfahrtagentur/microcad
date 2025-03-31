// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad literal syntax elements

mod color;
mod number_literal;
mod units;

pub use color::*;
pub use number_literal::*;
pub use units::*;

use crate::{r#type::*, src_ref::*, syntax::*, value::Value};

/// Literal entity
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    /// Integer literal
    Integer(Refer<i64>),
    /// Number literal
    Number(NumberLiteral),
    /// Boolean literal
    Bool(Refer<bool>),
    /// Color literal
    Color(Refer<Color>),
}

impl Literal {
    /// Return value of literal
    pub fn value(&self) -> Value {
        match self {
            Self::Integer(refer) => Value::Integer(refer.clone()),
            Self::Number(number_literal) => number_literal.value(),
            Self::Bool(refer) => Value::Bool(refer.clone()),
            Self::Color(refer) => Value::Color(refer.clone()),
        }
    }
}

impl SrcReferrer for Literal {
    fn src_ref(&self) -> SrcRef {
        match self {
            Literal::Number(n) => n.src_ref(),
            Literal::Integer(i) => i.src_ref(),
            Literal::Bool(b) => b.src_ref(),
            Literal::Color(c) => c.src_ref(),
        }
    }
}

impl crate::ty::Ty for Literal {
    fn ty(&self) -> Type {
        match self {
            Literal::Integer(_) => Type::Integer,
            Literal::Number(n) => n.ty(),
            Literal::Bool(_) => Type::Bool,
            Literal::Color(_) => Type::Color,
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Literal::Integer(i) => write!(f, "{}", i),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Color(c) => write!(f, "{}", c),
        }
    }
}

impl From<Literal> for Value {
    fn from(literal: Literal) -> Self {
        literal.value()
    }
}

impl PrintSyntax for Literal {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        write!(f, "{:depth$}Literal: ", "")?;
        match self {
            Literal::Integer(i) => writeln!(f, "{}", i),
            Literal::Number(n) => writeln!(f, "{}", n),
            Literal::Bool(b) => writeln!(f, "{}", b),
            Literal::Color(c) => writeln!(f, "{}", c),
        }
    }
}
