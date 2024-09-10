// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parse `call_argument_list` rule into CallArgumentList

use crate::{errors::*, eval::*, ord_map::*, parse::*, parser::*, src_ref::*};

/// List of call arguments
#[derive(Clone, Debug, Default)]
pub struct CallArgumentList(Refer<OrdMap<Identifier, CallArgument>>);

impl SrcReferrer for CallArgumentList {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

impl std::ops::Deref for CallArgumentList {
    type Target = OrdMap<Identifier, CallArgument>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for CallArgumentList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Eval for CallArgumentList {
    type Output = CallArgumentValueList;

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        let mut call_argument_list = CallArgumentValueList::default();

        for arg in self.iter() {
            call_argument_list
                .push(arg.eval(context)?)
                .map_err(EvalError::DuplicateCallArgument)?;
        }

        Ok(call_argument_list)
    }
}

impl Parse for CallArgumentList {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let mut call_argument_list =
            CallArgumentList(Refer::new(OrdMap::default(), pair.clone().into()));

        match pair.as_rule() {
            Rule::call_argument_list => {
                for pair in pair.clone().into_inner() {
                    call_argument_list
                        .push(CallArgument::parse(pair.clone())?)
                        .map_err(ParseError::DuplicateCallArgument)?;
                }

                Ok(call_argument_list)
            }
            rule => {
                unreachable!("CallArgumentList::parse expected call argument list, found {rule:?}")
            }
        }
    }
}

