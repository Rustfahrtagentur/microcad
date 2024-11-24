// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parse `call_argument_list` rule into CallArgumentList

use crate::{
    diag::PushDiag, errors::*, eval::*, ord_map::*, parse::*, parser::*, r#type::Type, src_ref::*,
};

/// List of call arguments
#[derive(Clone, Debug, Default)]
pub struct CallArgumentList(Refer<OrdMap<Identifier, CallArgument>>);

use strum::Display;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MatchError {
    /// Duplicated argument
    #[error("Duplicated argument: {0}")]
    DuplicatedArgument(Id),
    /// Occurs when a parameter was given in a call but not in the definition
    #[error("Parameter `{0}` ist not defined.")]
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
    ) -> Result<ArgumentMap> {
        let parameter_values = parameters.eval(context)?;
        self.eval(context)?
            .get_matching_arguments(&parameter_values)
    }

    // Get all matching arguments as an `ArgumentMap` from a parameter list definition
    /*pub fn get_matching_arguments(
        &self,
        context: &mut Context,
        parameters: &ParameterList,
    ) -> std::result::Result<ArgumentMap, MatchError> {
        let mut arg_map = ArgumentMap::default();
        let mut parameter_values = parameters.eval(context)?;

        // Fill the arg_map with default values of parameters
        for parameter in parameter_values.iter() {
            if let Some(default_value) = &parameter.default_value {
                arg_map.insert(parameter.name.clone(), default_value.clone());
            }
        }

        let mut positional_args = Vec::new();

        for arg in self.iter() {
            match &arg.derived_name() {
                // Named call argument, e.g. `a = 5.0`. `a` is the identifier
                Some(identifier) => {
                    let name = identifier.id().unwrap();
                    if arg_map.contains_key(&name) {
                        return Err(MatchError::DuplicatedArgument(name));
                    }

                    // Check if parameter `a` was defined
                    match parameter_values.get(&name) {
                        Some(parameter) => {
                            arg_map.insert(name.clone(), arg.get_named_match(context, parameter)?);
                            parameter_values.remove(&name);
                        }
                        None => {
                            return Err(MatchError::ParameterNotDefined(name));
                        }
                    }
                }
                None => {
                    positional_args.push(arg.value.eval(context)?);
                }
            }
        }

        // Find matching positional arguments
        let mut iter = parameter_values.iter();
        for positional_arg in positional_args {
            if let Some(param) = iter.next() {
                if param.type_matches(&positional_arg.ty()) {
                    arg_map.insert(param.name.clone(), positional_arg);
                } else {
                    return Err(MatchError::PositionalArgumentTypeMismatch(
                        param.name,
                        param.specified_type.unwrap(),
                        positional_arg.ty(),
                    ));
                }
            } else {
                break;
            }
        }

        for parameter in parameters.iter() {
            if arg_map.get(&parameter.name.id().unwrap()).is_none() {
                return Err(MatchError::MissingParameter(
                    parameter.name.id().unwrap().clone(),
                ));
            }
        }

        Ok(arg_map)
    }*/

    /*pub fn get_match_multi(
        &self,
        context: &mut Context,
        parameters: &ParameterList,
    ) -> Result<Combinations<Value>> {
    }*/
}

impl Eval for CallArgumentList {
    type Output = CallArgumentValueList;

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        let mut args = CallArgumentValueList::default();
        for arg in self.iter() {
            args.push(arg.eval(context)?);
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
