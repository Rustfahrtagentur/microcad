// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A single call argument

use crate::{eval::*, ord_map::*, parse::*, parser::*, src_ref::*};

/// Call argument
#[derive(Clone, Debug)]
pub struct CallArgument {
    /// Name of the argument
    pub name: Option<Identifier>,
    /// Value of the argument
    pub value: Expression,
    /// Source code reference
    src_ref: SrcRef,
}

impl CallArgument {
    /// Returns the name, if self.name is some. If self.name is None, try to extract the name from the expression
    pub fn derived_name(&self) -> Option<Identifier> {
        match &self.name {
            Some(name) => Some(name.clone()),
            None => self.value.single_identifier(),
        }
    }
}

impl Eval for CallArgument {
    type Output = CallArgumentValue;

    fn eval(&self, context: &mut Context) -> EvalResult<Self::Output> {
        Ok(CallArgumentValue::new(
            self.id(),
            self.value.eval(context)?,
            self.src_ref.clone(),
        ))
    }
}

impl SrcReferrer for CallArgument {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Sym for CallArgument {
    fn id(&self) -> Option<microcad_core::Id> {
        self.derived_name().as_ref().map(|name| name.id().clone())
    }
}

impl OrdMapValue<Identifier> for CallArgument {
    fn key(&self) -> Option<Identifier> {
        self.name.clone()
    }
}

impl Parse for CallArgument {
    fn parse(pair: Pair) -> ParseResult<Self> {
        match pair.clone().as_rule() {
            Rule::call_named_argument => {
                let mut inner = pair.inner();
                let first = inner.next().expect(INTERNAL_PARSE_ERROR);
                let second = inner.next().expect(INTERNAL_PARSE_ERROR);

                Ok(CallArgument {
                    name: Some(Identifier::parse(first)?),
                    value: Expression::parse(second)?,
                    src_ref: pair.src_ref(),
                })
            }
            Rule::expression => Ok(CallArgument {
                name: None,
                value: Expression::parse(pair.clone())?,
                src_ref: pair.into(),
            }),
            rule => unreachable!("CallArgument::parse expected call argument, found {rule:?}"),
        }
    }
}

impl std::fmt::Display for CallArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.name {
            Some(ref name) => write!(f, "{} = {}", name, self.value),
            None => write!(f, "{}", self.value),
        }
    }
}
