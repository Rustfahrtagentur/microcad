// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Argument match trait

use crate::eval::*;

/// Matching of `ParameterList` with `ArgumentValueList` into Tuple
#[derive(Default)]
pub struct ArgumentMatch<'a> {
    arguments: Vec<(&'a Identifier, &'a ArgumentValue)>,
    parameters: Vec<(&'a Identifier, &'a ParameterValue)>,
    result: Tuple,
}

impl<'a> ArgumentMatch<'a> {
    /// Match a `ParameterList` with an `ArgumentValueList` into an Tuple
    ///
    /// Returns `Ok(Tuple)`` if matches or Err() if not
    pub fn find_match(
        arguments: &'a ArgumentValueList,
        parameters: &'a ParameterValueList,
    ) -> EvalResult<Tuple> {
        let mut am = Self {
            arguments: arguments.iter().collect(),
            parameters: parameters.iter().collect(),
            result: Default::default(),
        };
        am.match_ids();
        am.match_types();
        am.match_defaults();

        am.check_missing()?;

        Ok(am.result)
    }

    /// Match a `ParameterList` with an `ArgumentValueList` into an Tuple
    ///
    /// Returns `Ok(Tuple)`` if matches or Err() if not
    pub fn find_multi_match(
        arguments: &'a ArgumentValueList,
        parameters: &'a ParameterValueList,
    ) -> EvalResult<Vec<Tuple>> {
        let mut am = Self {
            arguments: arguments.iter().collect(),
            parameters: parameters.iter().collect(),
            result: Default::default(),
        };
        am.match_ids();
        am.match_types();
        am.match_defaults();

        am.check_missing()?;

        Ok(am.multiply(parameters))
    }

    fn match_ids(&mut self) {
        if !self.arguments.is_empty() {
            log::trace!("find id match for:\n{self}");
            self.arguments.retain(|(id, arg)| {
                if !id.is_empty() {
                    if let Some(n) = self.parameters.iter().position(|(i, _)| i == id) {
                        let (id, _) = self.parameters.swap_remove(n);
                        log::trace!("found parameter by id: {id}");
                        self.result.insert((*id).clone(), arg.value.clone());
                        return false;
                    }
                }
                true
            });
        }
    }

    fn match_types(&mut self) {
        if !self.arguments.is_empty() {
            log::trace!("find type match for:\n{self}");
            self.arguments.retain(|(_, arg)| {
                if let Some(n) = self
                    .parameters
                    .iter()
                    .position(|(_, param)| param.ty() == arg.ty() || param.ty() == arg.ty_inner())
                {
                    let (id, _) = self.parameters.swap_remove(n);
                    self.result.insert((*id).clone(), arg.value.clone());
                    return false;
                }
                true
            })
        }
    }

    fn match_defaults(&mut self) {
        if !self.parameters.is_empty() {
            log::trace!("find default match for:\n{self}");
            // remove missing that can be found
            self.parameters.retain(|(id, param)| {
                // check for any default value
                if let Some(def) = &param.default_value {
                    // paranoia check if type is compatible
                    if def.ty() == param.ty() {
                        log::trace!("found argument by default: {id} = {def}");
                        self.result.insert((*id).clone(), def.clone());
                        return false;
                    }
                }
                true
            })
        }
    }

    fn check_missing(&self) -> EvalResult<()> {
        if !self.parameters.is_empty() {
            Err(EvalError::MissingArguments(
                self.parameters
                    .iter()
                    .map(|(id, _)| (*id).clone())
                    .collect(),
            ))
        } else if !self.arguments.is_empty() {
            Err(EvalError::TooManyArguments(
                self.arguments.iter().map(|(id, _)| (*id).clone()).collect(),
            ))
        } else {
            Ok(())
        }
    }

    fn multiply(&mut self, params: &ParameterValueList) -> Vec<Tuple> {
        let ids: std::collections::HashSet<_> = Self::multipliers(&self.result, params);
        if !ids.is_empty() {
            let mut result = Vec::new();
            self.result.multiplicity(ids, |t| {
                log::error!("multiplied: {t}");
                result.push(t)
            });
            result
        } else {
            vec![self.result.clone()]
        }
    }

    /// Return the multipliers' ids in the arguments.
    pub fn multipliers(
        args: &impl ValueAccess,
        params: &ParameterValueList,
    ) -> std::collections::HashSet<Identifier> {
        params
            .iter()
            .filter_map(|(id, param)| {
                if let Some(a) = args.by_id(id) {
                    if a.ty().is_list_of(&param.ty()) {
                        return Some(id);
                    }
                }
                None
            })
            .cloned()
            .collect()
    }
}

impl std::fmt::Display for ArgumentMatch<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "   Arguments: {}\n  Parameters: {}",
            self.arguments
                .iter()
                .map(|(id, arg)| format!("{id}: {arg}"))
                .collect::<Vec<_>>()
                .join(", "),
            self.parameters
                .iter()
                .map(|(id, param)| format!("{id}: {param}"))
                .collect::<Vec<_>>()
                .join(", "),
        )
    }
}

#[test]
fn argument_matching() {
    let parameters: ParameterValueList = [
        crate::parameter!(a: Scalar),
        crate::parameter!(b: Length),
        crate::parameter!(c: Scalar),
        crate::parameter!(d: Length = 4.0),
    ]
    .into_iter()
    .collect();

    let arguments: ArgumentValueList = [
        crate::argument!(a: Scalar = 1.0),
        crate::argument!(b: Length = 2.0),
        crate::argument!(Scalar = 3.0),
    ]
    .into_iter()
    .collect();

    let result =
        ArgumentMatch::find_match(&arguments, &parameters).expect("expect valid arguments");

    assert_eq!(result, crate::tuple!("(a=1.0, b=2.0mm, c=3.0, d=4.0mm)"));
}

#[test]
fn argument_match_fail() {
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
