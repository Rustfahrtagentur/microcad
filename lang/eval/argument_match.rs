// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Argument match trait

use crate::eval::*;

/// Matching of `ParameterList` with `ArgumentValueList` into Tuple
#[derive(Default)]
pub struct ArgumentMatch<'a> {
    arguments: Vec<(&'a Identifier, &'a ArgumentValue)>,
    params: Vec<(&'a Identifier, &'a ParameterValue)>,
    result: Tuple,
}

impl<'a> ArgumentMatch<'a> {
    /// Match a `ParameterList` with an `ArgumentValueList` into a tuple.
    ///
    /// Returns `Ok(Tuple)`` if matches or Err() if not
    pub fn find_match(
        arguments: &'a ArgumentValueList,
        params: &'a ParameterValueList,
    ) -> EvalResult<Tuple> {
        let am = Self::new(arguments, params)?;
        am.check_exact_types(params)?;
        Ok(am.result)
    }

    /// Match a `ParameterList` with an `ArgumentValueList` into an vector of tuples.
    ///
    /// Returns `Ok(Tuple)`` if matches or Err() if not
    pub fn find_multi_match(
        arguments: &'a ArgumentValueList,
        params: &'a ParameterValueList,
    ) -> EvalResult<Vec<Tuple>> {
        Ok(Self::new(arguments, params)?.multiply(params))
    }

    /// Create new instance and start matching.
    fn new(arguments: &'a ArgumentValueList, params: &'a ParameterValueList) -> EvalResult<Self> {
        let mut am = Self {
            arguments: arguments.iter().map(|(id, v)| (id, v)).collect(),
            params: params.iter().collect(),
            result: Tuple::new_named(std::collections::HashMap::new(), arguments.src_ref()),
        };

        am.match_ids();
        am.match_types(true);
        am.match_defaults();
        am.match_types(false);
        am.check_missing()?;

        Ok(am)
    }

    /// Match arguments by id
    fn match_ids(&mut self) {
        if !self.arguments.is_empty() {
            log::trace!("find id match for:\n{self}");
            self.arguments.retain(|(id, arg)| {
                if !id.is_empty() {
                    if let Some(n) = self.params.iter().position(|(i, _)| i == id) {
                        let (id, _) = self.params.swap_remove(n);
                        log::trace!("found parameter by id: {id:?}");
                        self.result.insert((*id).clone(), arg.value.clone());
                        return false;
                    }
                }
                true
            });
        }
    }

    /// Match arguments by type
    fn match_types(&mut self, mut exclude_defaults: bool) {
        if !self.arguments.is_empty() {
            if exclude_defaults {
                log::trace!("find type matches for (defaults):\n{self}");
            } else {
                log::trace!("find type matches for:\n{self}");
            }
            self.arguments.retain(|(arg_id, arg)| {
                // filter params by type
                let same_type: Vec<_> = self
                    .params
                    .iter()
                    .enumerate()
                    .filter(|(_, (_, _))| arg_id.is_empty())
                    .filter_map(|(n, (id, param))| {
                        if [Type::Invalid, arg.ty(), arg.ty_inner()].contains(&param.ty()) {
                            Some((n, id, param))
                        } else {
                            None
                        }
                    })
                    .collect();

                // if type check is exact ignore exclusion
                if same_type.len() == 1 {
                    exclude_defaults = false;
                }
                // ignore params with defaults
                let mut same_type = same_type
                    .iter()
                    .filter(|(_, _, param)| !exclude_defaults || param.default_value.is_none());

                if let Some((n, id, _)) = same_type.next() {
                    if same_type.next().is_none() {
                        log::trace!("found parameter by type: {id:?}");
                        self.result.insert((**id).clone(), arg.value.clone());
                        self.params.swap_remove(*n);
                        return false;
                    } else {
                        log::warn!("more than one parameter with that type")
                    }
                } else {
                    log::warn!("no parameter with that type")
                }
                true
            })
        }
    }

    /// Fill arguments with defaults
    fn match_defaults(&mut self) {
        if !self.params.is_empty() {
            log::trace!("find default match for:\n{self}");
            // remove missing that can be found
            self.params.retain(|(id, param)| {
                // check for any default value
                if let Some(def) = &param.default_value {
                    // paranoia check if type is compatible
                    if def.ty() == param.ty() {
                        log::trace!("found argument by default: {id:?} = {def}");
                        self.result.insert((*id).clone(), def.clone());
                        return false;
                    }
                }
                true
            })
        }
    }

    /// Return error if params are missing or arguments are to many
    fn check_missing(&self) -> EvalResult<()> {
        if !self.params.is_empty() {
            let mut missing: IdentifierList =
                self.params.iter().map(|(id, _)| (*id).clone()).collect();
            missing.sort();
            Err(EvalError::MissingArguments(missing))
        } else if !self.arguments.is_empty() {
            let mut too_many: IdentifierList =
                self.arguments.iter().map(|(id, _)| (*id).clone()).collect();
            too_many.sort();
            Err(EvalError::TooManyArguments(too_many))
        } else {
            Ok(())
        }
    }

    fn check_exact_types(&self, params: &ParameterValueList) -> EvalResult<()> {
        let multipliers = Self::multipliers(&self.result, params);
        if multipliers.is_empty() {
            return Ok(());
        }
        Err(EvalError::MultiplicityNotAllowed(multipliers))
    }

    /// Process parameter multiplicity
    ///
    /// Return one or many tuples.
    fn multiply(&self, params: &ParameterValueList) -> Vec<Tuple> {
        let ids: IdentifierList = Self::multipliers(&self.result, params);
        if !ids.is_empty() {
            let mut result = Vec::new();
            self.result.multiplicity(ids, |t| result.push(t));
            result
        } else {
            vec![self.result.clone()]
        }
    }

    /// Return the multipliers' ids in the arguments.
    fn multipliers(args: &impl ValueAccess, params: &ParameterValueList) -> IdentifierList {
        let mut result: IdentifierList = params
            .iter()
            .filter_map(|(id, param)| {
                if let Some(a) = args.by_id(id) {
                    if a.ty().is_array_of(&param.ty()) {
                        return Some(id);
                    }
                }
                None
            })
            .cloned()
            .collect();
        result.sort();
        result
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
            self.params
                .iter()
                .map(|(id, param)| format!("{id}: {param}"))
                .collect::<Vec<_>>()
                .join(", "),
        )
    }
}

#[test]
fn argument_matching() {
    let params: ParameterValueList = [
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

    let result = ArgumentMatch::find_match(&arguments, &params).expect("expect valid arguments");

    assert_eq!(result, crate::tuple!("(a=1.0, b=2.0mm, c=3.0, d=4.0mm)"));
}

#[test]
fn argument_match_fail() {
    let params: ParameterValueList = [
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
    assert!(ArgumentMatch::find_match(&arguments, &params).is_err());
}
