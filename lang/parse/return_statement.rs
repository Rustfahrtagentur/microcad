// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module statement parser entities
//!
use crate::{parse::*, parser::*, src_ref::*};

/// Module statement
#[derive(Clone, Debug)]
pub struct ReturnStatement {
    /// return value
    pub result: Option<Expression>,
    /// Source code reference
    src_ref: SrcRef,
}

impl SrcReferrer for ReturnStatement {
    fn src_ref(&self) -> SrcRef {
        self.src_ref
    }
}

impl Parse for ReturnStatement {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut result = None;

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::expression => result = Some(Expression::parse(pair)?),
                rule => unreachable!("Unexpected rule in return, got {:?}", rule),
            }
        }

        Ok(std::rc::Rc::new(ReturnStatement {
            result,
            src_ref: pair.into(),
        }))
    }
}

impl std::fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Module(m) => {
                write!(f, "{}", m.name)
            }
            Self::Namespace(ns) => {
                write!(f, "{}", ns.name)
            }
            Self::Function(f) => {
                write!(f, "{}", f.name)
            }
            Self::ModuleInit(mi) => {
                write!(f, "{mi}")
            }

            Self::Use(u) => write!(f, "{u}"),
            Self::Return(r) => write!(f, "{r}"),
            Self::If(i) => write!(f, "{i}"),
            Self::Marker(m) => write!(f, "{m}"),

            Self::Assignment(a) => write!(f, "{a}"),
            Self::Expression(e) => write!(f, "{e}"),
        }
    }
}
