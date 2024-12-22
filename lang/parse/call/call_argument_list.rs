// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parse `call_argument_list` rule into CallArgumentList

use crate::{diag::PushDiag, eval::*, ord_map::*, parse::*, parser::*, r#type::Type, src_ref::*};

/// List of call arguments
#[derive(Clone, Debug, Default)]
pub struct CallArgumentList(Refer<OrdMap<Identifier, CallArgument>>);

use thiserror::Error;

/// An error that occurred when looking for matching arguments between a call and a parameter definition
#[derive(Error, Debug)]
pub enum MatchError {
    /// Duplicated argument
    #[error("Duplicated argument: {0}")]
    DuplicatedArgument(Id),
    /// Occurs when a parameter was given in a call but not in the definition
    #[error("Parameter `{0}` is not defined.")]
    ParameterNotDefined(Id),
    /// Mismatching type
    #[error("Type mismatch for parameter `{0}`: expected `{1}`, got {2}")]
    PositionalArgumentTypeMismatch(Id, Type, Type),
    /// Parameter required by definition but given in the call
    #[error("Missing parameter: {0}")]
    MissingParameter(Id),
}

impl CallArgumentList {
    /// Get matching arguments from parameter list
    pub fn get_matching_arguments(
        &self,
        context: &mut Context,
        parameters: &ParameterList,
    ) -> EvalResult<ArgumentMap> {
        let parameter_values = parameters.eval(context)?;
        match self
            .eval(context)?
            .get_matching_arguments(&parameter_values)
        {
            Ok(args) => Ok(args),
            Err(err) => {
                context.error_with_stack_trace(self, err)?;
                Ok(ArgumentMap::default())
            }
        }
    }
}

impl Eval for CallArgumentList {
    type Output = CallArgumentValueList;

    fn eval(&self, context: &mut Context) -> EvalResult<Self::Output> {
        let mut args = CallArgumentValueList::default();
        for arg in self.iter() {
            args.push(arg.eval(context)?).expect("Duplicated argument");
        }
        Ok(args)
    }
}

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

impl Parse for CallArgumentList {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut call_argument_list =
            CallArgumentList(Refer::new(OrdMap::default(), pair.clone().into()));

        match pair.as_rule() {
            Rule::call_argument_list => {
                for pair in pair.inner() {
                    call_argument_list
                        .push(CallArgument::parse(pair)?)
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
