// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Argument match trait

use crate::{eval::*, value::*};

/// The `ArgumentMatch` trait is used to match arguments to parameters.
///
/// It is implemented by `ArgumentMap` and `MultiArgumentMap`.
pub trait ArgumentMatch: Default {
    /// Inserts a value into the map and removes it from the parameter list
    ///
    /// This function must be implemented by the user.
    fn insert_and_remove_from_parameters(
        &mut self,
        value: Value,
        parameter_value: &ParameterValue,
        parameter_values: &mut ParameterValueList,
    ) -> EvalResult<TypeCheckResult>;

    /// Find named arguments and insert them into the map.
    ///
    /// Finds all arguments with the same name as the parameter and inserts them into the map.
    /// Named arguments are arguments with a name, e.g. `bar` in `foo(bar = 42)`.
    /// The parameter is then removed from the list of parameters.
    fn find_and_insert_named_arguments(
        &mut self,
        argument_values: &ArgumentValueList,
        parameter_values: &mut ParameterValueList,
    ) -> EvalResult<&mut Self> {
        parameter_values
            // Clone the list of parameters because we want to remove elements from it while iterating
            .clone()
            .iter()
            // Filter out parameters that have a name and are present in the arguments
            .filter_map(|p| argument_values.get(&p.id).map(|c| (p, c)))
            // Insert the arguments into the map
            .try_for_each(|(parameter_value, argument_value)| {
                self.insert_and_remove_from_parameters(
                    argument_value.value.clone(),
                    parameter_value,
                    parameter_values,
                )?;
                Ok(())
            })
            // Return self
            .map(|_| self)
    }

    /// Find positional arguments and insert them into the map.
    ///
    /// Try to match arguments by their position and insert them into the map.
    /// Positional arguments are arguments without a name, e.g. `1, 2` in `foo(1, 2)`.
    /// The parameter is then removed from the list of parameters.
    fn find_and_insert_unnamed_arguments(
        &mut self,
        argument_values: &ArgumentValueList,
        parameter_values: &mut ParameterValueList,
    ) -> EvalResult<&mut Self> {
        if parameter_values.is_empty() {
            return Ok(self);
        }
        let mut positional_index = 0;

        for argument_value in argument_values.iter().filter(|arg| arg.id.is_none()) {
            let parameter_value = match parameter_values.get_by_index(positional_index) {
                Some(p) => p.clone(),
                None => break,
            };

            match self.insert_and_remove_from_parameters(
                argument_value.value.clone(),
                &parameter_value,
                parameter_values,
            )? {
                TypeCheckResult::MultiMatch | TypeCheckResult::SingleMatch => {
                    if positional_index >= parameter_values.len() {
                        break;
                    }
                }
                _ => {
                    positional_index += 1;
                }
            }
        }

        Ok(self)
    }

    /// Find default parameters and insert them into the map.
    ///
    /// If a parameter has a default value and is not present in the arguments, insert the default value.
    /// The parameter is then removed from the list of parameters.
    fn find_and_insert_default_parameters(
        &mut self,
        argument_values: &ArgumentValueList,
        parameter_values: &mut ParameterValueList,
    ) -> EvalResult<&mut Self> {
        parameter_values
            // Clone the list of parameters because we want to remove elements from it while iterating
            .clone()
            .iter()
            // Filter out parameters that have a default value and are not present in the arguments
            .filter_map(|p| match (argument_values.get(&p.id), &p.default_value) {
                (None, Some(default_value)) => Some((p, default_value.clone())),
                _ => None,
            })
            // Insert the default values into the map
            .try_for_each(|(parameter_value, default_value)| {
                self.insert_and_remove_from_parameters(
                    default_value,
                    parameter_value,
                    parameter_values,
                )?;
                Ok(())
            })
            // Return self
            .map(|_| self)
    }

    /// Find a match between arguments and parameters.
    ///
    /// This function tries to find a match between the arguments and the parameters.
    fn find_match(
        argument_values: &ArgumentValueList,
        parameter_values: &ParameterValueList,
    ) -> EvalResult<Self> {
        argument_values.check_for_unexpected_arguments(parameter_values)?;

        let mut missing_parameter_values = parameter_values.clone();
        let mut result = Self::default();

        result
            .find_and_insert_named_arguments(argument_values, &mut missing_parameter_values)?
            .find_and_insert_unnamed_arguments(argument_values, &mut missing_parameter_values)?
            .find_and_insert_default_parameters(argument_values, &mut missing_parameter_values)?;

        missing_parameter_values.check_for_missing_arguments()?;

        Ok(result)
    }
}
