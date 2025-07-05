// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Argument match trait

use crate::eval::*;

/// Matching of `ParameterList` with `ArgumentValueList` into ArgumentMap
#[derive(Default)]
pub struct ArgumentMatch<'a> {
    missing: Vec<&'a Identifier>,
    argument_map: ArgumentMap,
}

impl<'a> ArgumentMatch<'a> {
    /// Match a `ParameterList` with an `ArgumentValueList` into an ArgumentMap
    pub fn find_match(
        arguments: &ArgumentValueList,
        parameters: &'a ParameterValueList,
    ) -> EvalResult<ArgumentMap> {
        let mut am = Self {
            missing: parameters.keys().collect(),
            argument_map: Default::default(),
        };
        am.match_ids(arguments, parameters);
        am.match_types(arguments, parameters);
        am.match_defaults(parameters);
        am.match_multiplicity(arguments, parameters);
        am.check_missing()?;
        Ok(am.argument_map)
    }

    fn match_ids(&mut self, arguments: &ArgumentValueList, parameters: &ParameterValueList) {
        // remove missing that can be found
        self.missing.retain(|id| {
            // find  parameter by id
            if let Some(param) = &parameters.get(id) {
                // find argument by id
                if let Some(arg) = arguments.get(id) {
                    // check if type is compatible
                    if arg.value.ty() == param.ty() {
                        log::trace!("found argument by id: {id} : {ty}", ty = param.ty());
                        self.argument_map.insert((*id).clone(), arg.value.clone());
                        return false;
                    }
                }
            }
            true
        });
    }

    fn match_types(&mut self, arguments: &ArgumentValueList, parameters: &ParameterValueList) {
        // remove missing that can be found
        self.missing.retain(|id| {
            // find  parameter by id
            if let Some(param) = &parameters.get(id) {
                // find argument by type
                if let Some((_, arg)) = arguments.get_by_type(&param.ty()) {
                    log::trace!("found argument by type: {id} : {ty}", ty = param.ty());
                    self.argument_map.insert((*id).clone(), arg.value.clone());
                    return false;
                }
            }
            true
        });
    }

    fn match_defaults(&mut self, parameters: &ParameterValueList) {
        // remove missing that can be found
        self.missing.retain(|id| {
            // find  parameter by id
            if let Some(param) = &parameters.get(id) {
                // check for any default value
                if let Some(def) = &param.default_value {
                    // paranoia check if type is compatible
                    if def.ty() == param.ty() {
                        log::trace!("found argument by default: {id} = {def}");
                        self.argument_map.insert((*id).clone(), def.clone());
                        return false;
                    }
                }
            }
            true
        })
    }

    fn match_multiplicity(
        &mut self,
        arguments: &ArgumentValueList,
        parameters: &ParameterValueList,
    ) {
        // remove missing that can be found
        self.missing.retain(|id| {
            // find  parameter by id
            if let Some(param) = &parameters.get(id) {
                // find argument by id
                if let Some(arg) = arguments.get(id) {
                    // check if type is a list type
                    if let Type::List(ty) = arg.value.ty() {
                        // check if type is compatible
                        if ty.ty() == param.ty() {
                            log::trace!(
                                "found list argument by id: [{id}] = {ty}",
                                ty = param.ty()
                            );
                            self.argument_map.insert((*id).clone(), arg.value.clone());
                            return false;
                        }
                    }
                }
            }
            true
        })
    }

    fn check_missing(&self) -> EvalResult<()> {
        if self.missing.is_empty() {
            Ok(())
        } else {
            Err(EvalError::MissingArguments(
                self.missing.iter().map(|p| (*p).clone()).collect(),
            ))
        }
    }
}

#[test]
fn argument_matching() {
    let parameters: ParameterValueList = [
        crate::parameter!(x: Scalar),
        crate::parameter!(y: Length),
        crate::parameter!(z: Area = 1.0),
    ]
    .into_iter()
    .collect();
    let arguments: ArgumentValueList = [
        crate::argument!(x: Scalar = 1.0),
        crate::argument!(Length = 1.0),
    ]
    .into_iter()
    .collect();

    let result =
        ArgumentMatch::find_match(&arguments, &parameters).expect("expect valid arguments");

    assert_eq!(
        result,
        [
            crate::property!(x : Scalar = 1.0),
            crate::property!(y : Length = 1.0),
            crate::property!(z : Area = 1.0),
        ]
        .into_iter()
        .collect()
    );
}

#[test]
fn argument_matching_fail() {
    let parameters: ParameterValueList = [
        crate::parameter!(x: Scalar),
        crate::parameter!(y: Length),
        crate::parameter!(z: Area),
    ]
    .into_iter()
    .collect();
    let arguments: ArgumentValueList = [
        crate::argument!(x: Scalar = 1.0),
        crate::argument!(Length = 1.0),
    ]
    .into_iter()
    .collect();
    assert!(ArgumentMatch::find_match(&arguments, &parameters).is_err());
}
