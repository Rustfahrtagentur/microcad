// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parse `call_argument_list` rule into CallArgumentList

use std::any::Any;

use crate::{diag::PushDiag, errors::*, eval::*, ord_map::*, parse::*, parser::*, src_ref::*};

/// List of call arguments
#[derive(Clone, Debug, Default)]
pub struct CallArgumentList(Refer<OrdMap<Identifier, CallArgument>>);

impl CallArgumentList {
    pub fn get_matching_arguments(
        &self,
        context: &mut Context,
        parameters: &ParameterList,
    ) -> Result<ArgumentMap> {
        let mut arg_map = ArgumentMap::default();
        todo!();
        /*
        for arg in self.iter() {
            match &arg.name {
                // Named call argument, e.g. `a = 5.0`
                Some(name) => {
                    let name = name.id().unwrap();
                    if arg_map.contains_key(&name) {
                        context.error(self, anyhow::anyhow!("Duplicated argument: {name}"))?;
                        break;
                    }
                    // There must be a parameter in the parameter definition with the same name
                    if let Some(parameter) = parameters.get(name) {
                        let arg_value = arg.value.eval(context)?;
                        let param_value = parameter.eval(context)?;
                        if arg_value.ty() != param_value.ty() {
                            use crate::diag::PushDiag;
                            context.error(self, anyhow::anyhow!(""))
                        }

                        arg_map.insert(name, arg.value.eval(context)?);
                    }

                }
                None => match &arg.value.single_identifier() {
                    Some(name) => {
                        if let Some(parameter) = parameters.get(name) {
                            let name = name.id().unwrap();
                            arg_map.insert(name, arg.value.eval(context)?);
                        }
                    }
                    None => {
                        context.error(
                            self,
                            anyhow::anyhow!("Positional argument: {value}", value = arg.value),
                        )?;
                        // TODO: Handle positional arguments
                    }
                },
            }
        }

        Ok(arg_map)
        */
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
