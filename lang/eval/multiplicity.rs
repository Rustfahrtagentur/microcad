// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, rc::*, syntax::*, value::*};
use std::collections::HashMap;

/// Iterator over all combinations of given [`Multiplicity`].
pub struct MultiplicityIterator<'a> {
    /// Iterator target.
    multiplicity: &'a Multiplicity,
    /// Current combination (indizes of all parameters).
    indizes: HashMap<Identifier, usize>,
    /// `true` if there are no more combinations`.
    at_end: bool,
}

impl<'a> MultiplicityIterator<'a> {
    /// Return iterator over all combinations  of `multiplicity`.
    fn new(multiplicity: &'a Multiplicity) -> Self {
        let mut indizes = HashMap::new();
        multiplicity.args.iter().for_each(|(id, _)| {
            indizes.insert(id.clone(), 0);
        });
        Self {
            multiplicity,
            indizes,
            at_end: false,
        }
    }
}

impl Iterator for MultiplicityIterator<'_> {
    type Item = SymbolMap;

    fn next(&mut self) -> Option<Self::Item> {
        // cancel if at end
        if self.at_end {
            return None;
        }

        // create symbol map with symbols for current argument combination inside
        let mut map = SymbolMap::new();
        self.indizes.iter().for_each(|(id, pos)| {
            map.insert(
                id.clone(),
                Symbol::new(
                    SymbolDefinition::Constant(
                        id.clone(),
                        self.multiplicity.args[id][*pos].clone(),
                    ),
                    None,
                ),
            );
        });

        // proceed to next iteration
        let mut counted = false;
        for (id, pos) in self.indizes.iter_mut() {
            if *pos == self.multiplicity.args[id].len() - 1 {
                // reset if at maximum
                *pos = 0;
            } else {
                // count if not at maximum
                *pos += 1;
                counted = true;
                break;
            }
        }

        // if we did not count then there is nothing left
        if !counted {
            self.at_end = true;
        }

        // return symbol map
        Some(map)
    }
}

/// Parameter multiplicity combinator.
///
/// Set of one or maybe multiple arguments combinations which can be iterated with [`MultiplicityIterator`].
#[derive(Default, Debug)]
pub struct Multiplicity {
    args: HashMap<Identifier, Vec<Value>>,
}

impl Multiplicity {
    /// Create new multiplicity from *parameters* and *arguments*.
    ///
    /// Will search for every argument:
    /// 1) for a *parameter* which matches this *argument*'s *type* and *id* if given
    /// 2) for a *parameter* which matches this *argument*'s *list type* and *id* if given
    /// 3) for a *parameter* which matches this *argument*'s *type* if exactly one
    /// 4) for a *parameter* which matches this *argument*'s *list type* if exactly one
    ///
    /// Will return error:
    /// - [`EvalError::ParameterTypeMismatch`]: if the *parameter type* is not compatible with the *argument type* when given by *id*.
    /// - [`EvalError::AmbiguousArgument`]: if multiple *parameters* match an *argument*
    /// - [`EvalError::MissingParameter`]: if a *parameter* does not match any *argument*
    /// - [`EvalError::ParameterNotFound`]: if an *argument* does not match any *parameter*
    pub fn new(parameters: &ParameterValueList, args: &CallArgumentValueList) -> EvalResult<Self> {
        // resulting multiplicity
        let mut result = Self::default();
        // we remove found parameters form this to see if any are left
        let mut missing = parameters.clone();
        // find a matching parameter for every argument
        for arg in args.iter() {
            // if argument id is given search for a matching parameter
            if let Some(id) = &arg.id {
                // announce (1) + (2)
                log::trace!("getting argument by id: '{}'", id);
                // find parameter with that name
                if let Some(parameter) = missing.get_by_id(id) {
                    log::trace!("found parameter by id '{id}': {parameter}");
                    // check if type is exactly the same or a list of it
                    if arg.value.ty().can_convert_into(&parameter.ty()) {
                        // found (1)
                        log::trace!(
                            "found single value by id '{id}': {} = {}",
                            parameter.ty(),
                            arg.value
                        );
                        // insert single value
                        result
                            .args
                            .insert(parameter.id.clone(), vec![arg.value.clone()]);
                        // remove from missing parameters
                        missing.remove(id);
                        // continue with next argument
                        continue;
                    } else if let Value::List(list) = &arg.value {
                        // check if list type matches exactly
                        if list.inner_ty().can_convert_into(&parameter.ty()) {
                            // found (2)
                            log::trace!(
                                "found multiple values by id '{id}': {} = {list}",
                                parameter.ty()
                            );
                            // insert multiple values
                            result.args.insert(parameter.id.clone(), list.fetch());
                            // remove from missing parameters
                            missing.remove(&parameter.id);
                            // continue with next argument
                            continue;
                        }
                    } else {
                        return Err(EvalError::ParameterTypeMismatch {
                            id: id.clone(),
                            expected: parameter.ty(),
                            found: arg.value.ty(),
                        });
                    }
                }
                return Err(EvalError::ParameterNotFound(id.clone()));
            }

            let ty = arg.value.ty();
            // announce (3)
            log::debug!("getting argument by type: '{ty}'");
            // after matching by id we now need a type for further matching
            if ty == Type::Invalid {
                return Err(EvalError::InvalidArgumentType(arg.clone()));
            }

            // find parameter that matches argument type
            let parameter_values = missing.get_by_type(&ty);
            match parameter_values.len() {
                0 => log::warn!("could not find parameter by type '{ty}'"),
                1 => {
                    let value = &parameter_values.first().expect("single element");
                    log::trace!(
                        "found value by type '{}': {} = {}",
                        value.id,
                        value.ty(),
                        arg.value
                    );
                    // insert single value
                    result
                        .args
                        .insert(value.id.clone(), vec![arg.value.clone()]);
                    // remove from missing parameters
                    missing.remove(&value.id);
                    // continue with next argument
                    continue;
                }
                2.. => {
                    return Err(EvalError::AmbiguousArgument(parameter_values.into()));
                }
            }

            // find parameter that matches multiplied argument type
            if let Value::List(list) = &arg.value {
                let ty = list.inner_ty();
                // announce (4)
                log::debug!("getting argument by multiple type: '{ty}'");

                // find parameter by type
                let parameter_values = missing.get_by_type(&ty);
                match parameter_values.len() {
                    0 => todo!("error: could not find parameter by type {ty} or multiple [{ty}]"),
                    1 => {
                        let value = &parameter_values.first().expect("single element");
                        log::trace!(
                            "found multiple by type '{}': {} = {}",
                            value.id,
                            value.ty(),
                            arg.value
                        );
                        // insert multiple values
                        result.args.insert(value.id.clone(), list.fetch());
                        // remove from missing parameters
                        missing.remove(&value.id);
                        // continue with next argument
                        continue;
                    }
                    2.. => {
                        return Err(EvalError::AmbiguousArgument(parameter_values.into()));
                    }
                }
            }
        }
        let missing: Vec<Rc<ParameterValue>> = missing
            .iter()
            .filter(|parameter| {
                if let Some(value) = &parameter.default_value {
                    log::trace!("using default {value} for {parameter}");
                    result
                        .args
                        .insert(parameter.id.clone(), vec![value.clone()]);
                    false
                } else {
                    true
                }
            })
            .cloned()
            .collect();

        if !missing.is_empty() {
            return Err(EvalError::MissingParameter(missing.into()));
        }
        Ok(result)
    }

    /// Process a (maybe multiple) call.
    pub fn call(
        &self,
        mut single_f: impl FnMut(SymbolMap) -> EvalResult<Value>,
    ) -> EvalResult<Value> {
        match self.len() {
            0 => todo!("multiplicity error"),
            1 => single_f(self.iter().next().expect("exact one value")),
            _ => {
                let mut result = Vec::new();
                for symbols in self.iter() {
                    result.push(single_f(symbols)?);
                }
                Ok(result.into())
            }
        }
    }

    /// Return iterator over all combinations.
    pub fn iter(&self) -> MultiplicityIterator {
        MultiplicityIterator::new(self)
    }

    /// Return the number of combinations.
    pub fn len(&self) -> usize {
        self.args.values().map(|vec| vec.len()).product()
    }

    /// Returns true, if there are no available combinations.
    pub fn is_empty(&self) -> bool {
        self.args.is_empty()
    }
}
