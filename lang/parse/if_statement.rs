// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! If statement parser entities
//!
use crate::{parse::*, parser::*, src_ref::*};

/// If statement
#[derive(Clone, Debug)]
pub struct If {
    /// if condition
    pub cond: Expression,
    /// body if true
    pub body: Body,
    /// body if false
    pub body_else: Option<Body>,
    /// Source code reference
    src_ref: SrcRef,
}

impl SrcReferrer for If {
    fn src_ref(&self) -> SrcRef {
        self.src_ref
    }
}

impl Parse for If {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut cond = None;
        let mut body = Body::default();
        let mut body_else = None;

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::expression => cond = Some(Expression::parse(pair)?),
                Rule::body => Some(ParameterList::parse(pair)?),
                Rule::body_else => {
                    body_else = Body::parse(pair.clone())?;
                }
                rule => unreachable!("Unexpected rule in if, got {:?}", rule),
            }
        }

        Ok(std::rc::Rc::new(If {
            cond,
            body,
            body_else,
            src_ref: pair.into(),
        }))
    }
}

impl Eval for If {
    type Output = Option<Value>;

    fn eval(&self, context: &mut EvalContext) -> std::result::Result<Self::Output, EvalError> {
        todo!()
    }
}

impl std::fmt::Display for If {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        todo!()
    }
}
