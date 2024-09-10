// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µCAD assignment parser entity

use crate::{
    errors::*,
    eval::*,
    parse::*,
    parser::*,
    r#type::*,
    src_ref::{SrcRef, SrcReferrer},
};

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

impl SrcReferrer for Assignment {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Parse for Assignment {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let mut name = Identifier::default();
        let mut specified_type = None;
        let mut value = None;
        
        for pair in pair.clone().into_inner() {
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
            value: value.unwrap(),
            src_ref: pair.into(),
        })
    }
}

impl Eval for Assignment {
    type Output = ();

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        let value = self.value.eval(context)?;
        context.add_value(self.name.id().expect("nameless lvalue"), value);
        Ok(())
    }
}

impl std::fmt::Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.specified_type {
            Some(t) => write!(f, "{}: {} = {}", self.name, t.ty(), self.value),
            None => write!(f, "{} = {}", self.name, self.value),
        }
    }
}

