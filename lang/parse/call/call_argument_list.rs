// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parse `call_argument_list` rule into CallArgumentList

use std::any::{self, Any};

use crate::{diag::PushDiag, errors::*, eval::*, ord_map::*, parse::*, parser::*, src_ref::*};

/// List of call arguments
#[derive(Clone, Debug, Default)]
pub struct CallArgumentList(Refer<OrdMap<Identifier, CallArgument>>);

impl CallArgumentList {
    /// Get all matching arguments as an `ArgumentMap` from a parameter list definition
    pub fn get_matching_arguments(
        &self,
        context: &mut Context,
        parameters: &ParameterList,
    ) -> Result<ArgumentMap> {
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
                        context.error(self, anyhow::anyhow!("Duplicated argument: {name}"))?;
                        break;
                    }

                    // Check if parameter `a` was defined
                    match parameter_values.get(&name) {
                        Some(parameter) => {
                            arg_map.insert(name.clone(), arg.get_named_match(context, parameter)?);
                            parameter_values.remove(&name);
                        }
                        None => {
                            context.error(
                                self,
                                anyhow::anyhow!("Parameter `{name}` ist not defined."),
                            )?;
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
                    context.error(self, anyhow::anyhow!("Parameter type mismatch"))?;
                }
            } else {
                break;
            }
        }

        for parameter in parameters.iter() {
            if arg_map.get(&parameter.name.id().unwrap()).is_none() {
                context.error(
                    self,
                    anyhow::anyhow!("Parameter {name} is missing.", name = &parameter.name),
                )?;
            }
        }

        Ok(arg_map)
    }

    /*pub fn get_match_multi(
        &self,
        context: &mut Context,
        parameters: &ParameterList,
    ) -> Result<Combinations<Value>> {
    }*/
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
