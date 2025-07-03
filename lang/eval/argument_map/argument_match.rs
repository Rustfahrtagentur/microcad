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
        id: &Identifier,
        parameter: &ParameterValue,
        parameters: &mut ParameterValueList,
    ) -> EvalResult<TypeCheckResult>;

    /// Find named arguments and insert them into the map.
    ///
    /// Finds all arguments with the same name as the parameter and inserts them into the map.
    /// Named arguments are arguments with a name, e.g. `bar` in `foo(bar = 42)`.
    /// The parameter is then removed from the list of parameters.
    fn find_and_insert_named_arguments(
        mut self,
        arguments: &ArgumentValueList,
        parameters: &mut ParameterValueList,
    ) -> EvalResult<Self> {
        parameters
            // Clone the list of parameters because we want to remove elements from it while iterating
            .clone()
            .iter()
            // Filter out parameters that have a name and are present in the arguments
            .filter_map(|(id, p)| arguments.get(&id).map(|c| (id, p, c)))
            // Insert the arguments into the map
            .try_for_each(|(id, parameter, argument)| {
                self.insert_and_remove_from_parameters(
                    argument.value.clone(),
                    id,
                    parameter,
                    parameters,
                )?;
                Ok(())
            })
            // Return self
            .map(|_| self)
    }

    /// Find arguments by type and insert them into the map.
    ///
    /// Try to match unnamed arguments by their type and insert them into the map.
    /// The parameter is then removed from the list of parameters.
    fn find_and_insert_unnamed_arguments(
        mut self,
        arguments: &ArgumentValueList,
        parameters: &mut ParameterValueList,
    ) -> EvalResult<Self> {
        let mut positional_index = 0;

        for (id, argument) in arguments.iter().filter(|(id, _)| id.is_empty()) {
            let parameter = match parameters.get_by_type(argument.value.ty()) {
                Ok(p) => p.clone(),
                Err(_) => break,
            };

            match self.insert_and_remove_from_parameters(
                argument.value.clone(),
                &parameter,
                parameters,
            )? {
                TypeCheckResult::MultiMatch | TypeCheckResult::SingleMatch => {
                    if positional_index >= parameters.len() {
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
        mut self,
        arguments: &ArgumentValueList,
        parameters: &mut ParameterValueList,
    ) -> EvalResult<Self> {
        parameters
            // Clone the list of parameters because we want to remove elements from it while iterating
            .clone()
            .iter()
            // Filter out parameters that have a default value and are not present in the arguments
            .filter_map(|(id, p)| match (arguments.get(&id), &p.default_value) {
                (None, Some(default_value)) => Some((p, default_value.clone())),
                _ => None,
            })
            // Insert the default values into the map
            .try_for_each(|(parameter_value, default_value)| {
                self.insert_and_remove_from_parameters(default_value, parameter_value, parameters)?;
                Ok(())
            })
            // Return self
            .map(|_| self)
    }

    /// Find a match between arguments and parameters.
    ///
    /// This function tries to find a match between the arguments and the parameters.
    fn find_match(
        arguments: &ArgumentValueList,
        parameters: &ParameterValueList,
    ) -> EvalResult<Self> {
        arguments.check_for_unexpected_arguments(parameters)?;

        let mut missing_parameter_values = parameters.clone();
        let result = Self::default()
            .find_and_insert_named_arguments(arguments, &mut missing_parameter_values)?
            .find_and_insert_unnamed_arguments(arguments, &mut missing_parameter_values)?
            .find_and_insert_default_parameters(arguments, &mut missing_parameter_values)?;

        missing_parameter_values.check_for_missing_arguments()?;

        Ok(result)
    }
}
