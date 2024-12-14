// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Argument map evaluation entity

use crate::{eval::*, parse::Combinations, src_ref::*};

pub trait ArgumentMatch: Default {
    fn contains_key(&self, key: &Id) -> bool;

    fn insert_and_remove_from_parameters(
        &mut self,
        value: Value,
        parameter_value: &ParameterValue,
        parameter_values: &mut ParameterValueList,
    ) -> TypeCheckResult;

    fn find_and_insert_named_arguments(
        &mut self,
        call_values: &CallArgumentValueList,
        parameter_values: &mut ParameterValueList,
    ) -> Result<&mut Self> {
        parameter_values.clone().iter().for_each(|parameter_value| {
            // We have found a matching call argument with the same name as the parameter.
            if let Some(call_argument_value) = call_values.get(&parameter_value.name) {
                self.insert_and_remove_from_parameters(
                    call_argument_value.value.clone(),
                    parameter_value,
                    parameter_values,
                );
            }
        });

        Ok(self)
    }

    fn find_and_insert_positional_arguments(
        &mut self,
        call_values: &CallArgumentValueList,
        parameter_values: &mut ParameterValueList,
    ) -> Result<&mut Self> {
        if parameter_values.is_empty() {
            return Ok(self);
        }
        let mut positional_index = 0;

        call_values
            .iter()
            .filter(|arg| arg.name.is_none())
            .try_for_each(|arg| {
                use std::ops::ControlFlow;
                if parameter_values.get_by_index(positional_index).is_none() {
                    ControlFlow::Break(())
                } else {
                    let parameter_value = parameter_values[positional_index].clone();
                    let parameter_name = parameter_value.name.clone();
                    if !self.contains_key(&parameter_name) {
                        match self.insert_and_remove_from_parameters(
                            arg.value.clone(),
                            &parameter_value,
                            parameter_values,
                        ) {
                            TypeCheckResult::MultiMatch | TypeCheckResult::SingleMatch => {
                                if positional_index >= parameter_values.len() {
                                    return ControlFlow::Break(());
                                }
                            }
                            _ => {}
                        }
                    } else {
                        positional_index += 1;
                    }
                    ControlFlow::Continue(())
                }
            });

        Ok(self)
    }

    fn find_and_insert_default_parameters(
        &mut self,
        call_values: &CallArgumentValueList,
        parameter_values: &mut ParameterValueList,
    ) -> Result<&mut Self> {
        parameter_values.clone().iter().for_each(|parameter_value| {
            if call_values.get(&parameter_value.name).is_none() {
                // If we have a default value, we can use it
                if let Some(default) = &parameter_value.default_value {
                    self.insert_and_remove_from_parameters(
                        default.clone(),
                        &parameter_value,
                        parameter_values,
                    );
                }
            }
        });

        Ok(self)
    }

    fn find_match(
        call_values: &CallArgumentValueList,
        parameter_values: &ParameterValueList,
    ) -> Result<Self> {
        call_values.check_for_unexpected_arguments(parameter_values)?;

        let mut missing_parameter_values = parameter_values.clone();
        let mut multi_arg_map = Self::default();

        multi_arg_map
            .find_and_insert_named_arguments(call_values, &mut missing_parameter_values)?
            .find_and_insert_positional_arguments(call_values, &mut missing_parameter_values)?
            .find_and_insert_default_parameters(call_values, &mut missing_parameter_values)?;

        missing_parameter_values.check_for_missing_arguments()?;

        Ok(multi_arg_map)
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
    fn contains_key(&self, key: &Id) -> bool {
        self.0.contains_key(key)
    }

    fn insert_and_remove_from_parameters(
        &mut self,
        value: Value,
        parameter_value: &ParameterValue,
        parameter_values: &mut ParameterValueList,
    ) -> TypeCheckResult {
        let name = &parameter_value.name;
        parameter_values.remove(name);
        self.insert(name.clone(), value.clone());
        TypeCheckResult::SingleMatch
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
    fn contains_key(&self, key: &Id) -> bool {
        self.0.contains_key(key)
    }

    /// Insert a value into the map and remove `parameter_value` from the list
    fn insert_and_remove_from_parameters(
        &mut self,
        value: Value,
        parameter_value: &ParameterValue,
        parameter_values: &mut ParameterValueList,
    ) -> TypeCheckResult {
        let result = parameter_value.type_check(&value.ty());
        let name = &parameter_value.name;
        match result {
            TypeCheckResult::MultiMatch => match &value {
                Value::List(l) => {
                    parameter_values.remove(name);
                    self.insert_multi(name.clone(), l.fetch());
                    result
                }
                value => panic!("Expected list type, got {}", value.ty()),
            },
            TypeCheckResult::SingleMatch => {
                parameter_values.remove(&name);
                self.insert_single(name.clone(), value);
                result
            }
            _ => result,
        }
    }
}
