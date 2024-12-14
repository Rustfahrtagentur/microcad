// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Argument map evaluation entity

use call::call_argument_value;

use crate::{eval::*, parse::Combinations, src_ref::*};

/// The `ArgumentMatch` trait is used to match call arguments to parameters.
///
/// It is implemented by `ArgumentMap` and `MultiArgumentMap`.
pub trait ArgumentMatch: Default {
    /// Check if we have an argument value with name
    ///
    /// This function must be implemented by the user.
    fn contains_argument_value(&self, name: &Id) -> bool;

    /// Inserts a value into the map and removes it from the parameter list
    ///
    /// This function must be implemented by the user.
    fn insert_and_remove_from_parameters(
        &mut self,
        value: Value,
        parameter_value: &ParameterValue,
        parameter_values: &mut ParameterValueList,
    ) -> Result<TypeCheckResult>;

    /// Find named arguments and insert them into the map.
    ///
    /// Finds all call arguments with the same name as the parameter and inserts them into the map.
    /// Named arguments are call arguments with a name, e.g. `bar` in `foo(bar = 42)`.
    /// The parameter is then removed from the list of parameters.
    fn find_and_insert_named_arguments(
        &mut self,
        call_argument_values: &CallArgumentValueList,
        parameter_values: &mut ParameterValueList,
    ) -> Result<&mut Self> {
        for (parameter_value, call_argument_value) in parameter_values
            .clone()
            .iter()
            .map(|p| (p, call_argument_values.get(&p.name)))
            .filter(|(_, call_argument_value)| call_argument_value.is_some())
            .map(|(p, c)| (p, c.unwrap().value.clone()))
        {
            // We have found a matching call argument with the same name as the parameter.
            self.insert_and_remove_from_parameters(
                call_argument_value,
                parameter_value,
                parameter_values,
            )?;
        }

        Ok(self)
    }

    /// Find positional arguments and insert them into the map.
    ///
    /// Try to match call arguments by their position and insert them into the map.
    /// Positional arguments are call arguments without a name, e.g. `1, 2` in `foo(1, 2)`.
    /// The parameter is then removed from the list of parameters.
    fn find_and_insert_positional_arguments(
        &mut self,
        call_argument_values: &CallArgumentValueList,
        parameter_values: &mut ParameterValueList,
    ) -> Result<&mut Self> {
        if parameter_values.is_empty() {
            return Ok(self);
        }
        let mut positional_index = 0;

        for call_argument_value in call_argument_values.iter().filter(|arg| arg.name.is_none()) {
            let parameter_value = match parameter_values.get_by_index(positional_index) {
                Some(p) => p.clone(),
                None => break,
            };

            if !self.contains_argument_value(&parameter_value.name) {
                match self.insert_and_remove_from_parameters(
                    call_argument_value.value.clone(),
                    &parameter_value,
                    parameter_values,
                )? {
                    TypeCheckResult::MultiMatch | TypeCheckResult::SingleMatch => {
                        if positional_index >= parameter_values.len() {
                            break;
                        }
                    }
                    _ => {}
                }
            } else {
                positional_index += 1;
            }
        }

        Ok(self)
    }

    /// Find default parameters and insert them into the map.
    ///
    /// If a parameter has a default value and is not present in the call arguments, insert the default value.
    /// The parameter is then removed from the list of parameters.
    fn find_and_insert_default_parameters(
        &mut self,
        call_argument_values: &CallArgumentValueList,
        parameter_values: &mut ParameterValueList,
    ) -> Result<&mut Self> {
        for parameter_value in parameter_values.clone().iter() {
            if call_argument_values.get(&parameter_value.name).is_none() {
                // If we have a default value, we can use it
                if let Some(default) = &parameter_value.default_value {
                    self.insert_and_remove_from_parameters(
                        default.clone(),
                        &parameter_value,
                        parameter_values,
                    )?;
                }
            }
        }

        Ok(self)
    }

    /// Find a match between call arguments and parameters.
    ///
    /// This function tries to find a match between the call arguments and the parameters.
    fn find_match(
        call_argument_values: &CallArgumentValueList,
        parameter_values: &ParameterValueList,
    ) -> Result<Self> {
        call_argument_values.check_for_unexpected_arguments(parameter_values)?;

        let mut missing_parameter_values = parameter_values.clone();
        let mut result = Self::default();

        result
            .find_and_insert_named_arguments(call_argument_values, &mut missing_parameter_values)?
            .find_and_insert_positional_arguments(
                call_argument_values,
                &mut missing_parameter_values,
            )?
            .find_and_insert_default_parameters(
                call_argument_values,
                &mut missing_parameter_values,
            )?;

        missing_parameter_values.check_for_missing_arguments()?;

        Ok(result)
    }
}

/// Map of arguments
#[derive(Clone, Debug, Default)]
pub struct ArgumentMap(Refer<std::collections::HashMap<Id, Value>>);

impl ArgumentMap {
    /// Create empty argument map
    pub fn new(src_ref: SrcRef) -> Self {
        Self(Refer::new(std::collections::HashMap::new(), src_ref))
    }

    /// Fetch an argument by name
    pub fn get_value<'a, T>(&'a self, name: &str) -> T
    where
        T: std::convert::TryFrom<&'a Value>,
        T::Error: std::fmt::Debug,
    {
        self.0
            .get(name)
            .expect("no name found")
            .try_into()
            .expect("cannot convert argument value")
    }
}

impl SrcReferrer for ArgumentMap {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

impl std::ops::Deref for ArgumentMap {
    type Target = std::collections::HashMap<Id, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ArgumentMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ArgumentMatch for ArgumentMap {
    fn contains_argument_value(&self, key: &Id) -> bool {
        self.0.contains_key(key)
    }

    fn insert_and_remove_from_parameters(
        &mut self,
        value: Value,
        parameter_value: &ParameterValue,
        parameter_values: &mut ParameterValueList,
    ) -> Result<TypeCheckResult> {
        let name = &parameter_value.name;
        parameter_values.remove(name);
        self.insert(name.clone(), value.clone());
        Ok(TypeCheckResult::SingleMatch)
    }
}

/// An argument map for parameter multiplicity.
///
/// In the combination map, every value can be a single or multi coefficient.
/// Let's assume, you have a `module a(r: scalar) {}`:
/// * If you call `a(4.0)`, `a` will be stored as a single coefficient, because we passed a single scalar.
/// * If you call `a([2.0, 4.0])`, `a` will be stored as a multi coefficient, because we passed a list of scalars.
#[derive(Default)]
pub struct MultiArgumentMap(crate::parse::call::CombinationMap<Value>);

impl MultiArgumentMap {
    /// Insert a multi-value coefficient
    pub fn insert_multi(&mut self, name: Id, value: Vec<Value>) {
        self.0
            .insert(name, crate::parse::call::Coefficient::Multi(value));
    }

    /// Insert a single-value coefficient
    pub fn insert_single(&mut self, name: Id, value: Value) {
        self.0
            .insert(name, crate::parse::call::Coefficient::Single(value));
    }

    /// Return an iterator over all combinations
    pub fn combinations(&self) -> Combinations<Value> {
        Combinations::new(&self.0)
    }
}

impl ArgumentMatch for MultiArgumentMap {
    /// Check if the argument map contains a key
    fn contains_argument_value(&self, key: &Id) -> bool {
        self.0.contains_key(key)
    }

    /// Insert a value into the map and remove `parameter_value` from the list
    fn insert_and_remove_from_parameters(
        &mut self,
        value: Value,
        parameter_value: &ParameterValue,
        parameter_values: &mut ParameterValueList,
    ) -> Result<TypeCheckResult> {
        let result = parameter_value.type_check(&value.ty());
        let name = &parameter_value.name;
        match result {
            TypeCheckResult::MultiMatch => match &value {
                Value::List(l) => {
                    parameter_values.remove(name);
                    self.insert_multi(name.clone(), l.fetch());
                    Ok(result)
                }
                value => Err(EvalError::ExpectedIterable(value.ty().clone())),
            },
            TypeCheckResult::SingleMatch => {
                parameter_values.remove(&name);
                self.insert_single(name.clone(), value);
                Ok(result)
            }
            _ => Ok(result),
        }
    }
}
