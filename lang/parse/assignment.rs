// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad assignment parser entity

use crate::{eval::*, parse::*, parser::*, r#type::*, src_ref::*, sym::*};

/// Assignment specifying an identifier, type and value
#[derive(Clone, Debug)]
pub struct Assignment {
    /// Assignee
    pub name: Identifier,
    /// Type of the assignee
    pub specified_type: Option<TypeAnnotation>,
    /// Value to assign
    pub value: Expression,
    /// Source code reference
    src_ref: SrcRef,
}

impl Assignment {
    /// Make a symbol from the assignment
    pub fn make_symbol(&self, context: &mut EvalContext) -> EvalResult<Symbol> {
        Ok(Symbol::Value(
            self.name.id().clone(),
            self.value.eval(context)?,
        ))
    }
}

impl SrcReferrer for Assignment {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Parse for Assignment {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut name = Identifier::default();
        let mut specified_type = None;
        let mut value = None;

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::identifier => {
                    name = Identifier::parse(pair)?;
                }
                Rule::r#type => {
                    specified_type = Some(TypeAnnotation::parse(pair)?);
                }
                Rule::expression => {
                    value = Some(Expression::parse(pair)?);
                }
                rule => {
                    unreachable!("Unexpected token in assignment: {:?}", rule);
                }
            }
        }

        Ok(Self {
            name,
            specified_type,
            value: value.expect(INTERNAL_PARSE_ERROR),
            src_ref: pair.into(),
        })
    }
}

impl Eval for Assignment {
    type Output = Symbol;

    fn eval(&self, context: &mut EvalContext) -> EvalResult<Self::Output> {
        let symbol = self.make_symbol(context)?;
        context.add(symbol.clone());
        Ok(symbol)
    }
}

impl std::fmt::Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.specified_type {
            Some(t) => write!(f, "{}: {} = {}", self.name, t.ty(), self.value),
            None => write!(f, "{} = {}", self.name, self.value),
        }
    }
}
