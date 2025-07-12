// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Argument match trait

use crate::{eval::*, value::*};

/// The `ArgumentMatch` trait is used to match arguments to parameters.
///
/// It is implemented by `ArgumentMap` and `MultiArgumentMap`.
pub trait ArgumentMatch: Default + std::fmt::Display + SrcReferrer {
    /// Create new instance
    /// - `src_ref`: source code reference for argument list
    fn new(src_ref: SrcRef) -> Self;

    /// Inserts a value into the map and removes it from the parameter list
    ///
    /// This function must be implemented by the user.
    fn insert_and_remove_from_parameters(
        &mut self,
        id: &Identifier,
        value: Value,
        parameter_value: &ParameterValue,
        parameter_values: &mut ParameterValueList,
    ) -> EvalResult<TypeCheckResult>;

    /// Find named arguments and insert them into the map.
    ///
    /// Finds all arguments with the same name as the parameter and inserts them into the map.
    /// Named arguments are arguments with a name, e.g. `bar` in `foo(bar = 42)`.
    /// The parameter is then removed from the list of parameters.
    fn match_named(
        &mut self,
        argument_values: &ArgumentValueList,
        parameter_values: &mut ParameterValueList,
    ) -> EvalResult<&mut Self> {
        parameter_values
            // Clone the list of parameters because we want to remove elements from it while iterating
            .clone()
            .iter()
            // Filter out parameters that have a name and are present in the arguments
            .filter_map(|(id, p)| argument_values.get(id).map(|c| (id, p, c)))
            // Insert the arguments into the map
            .try_for_each(|(id, parameter_value, argument_value)| {
                log::trace!("found match by id: {id}");
                self.insert_and_remove_from_parameters(
                    id,
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
    fn match_unnamed(
        &mut self,
        argument_values: &ArgumentValueList,
        parameter_values: &mut ParameterValueList,
    ) -> EvalResult<()> {
        if parameter_values.is_empty() {
            return Ok(());
        }
        let mut idx = parameter_values.len();

        for (_, argument_value) in argument_values.iter() {
            let unnamed: Vec<_> = parameter_values
                .keys()
                .filter(|k| !argument_values.contains_key(k))
                .collect();
            let (id, parameter_value) =
                match parameter_values.get_by_type(argument_value.ty(), unnamed.into_iter()) {
                    Ok((id, p)) => (id.clone(), p.clone()),
                    Err(_) => continue,
                };

            assert!(!id.is_empty());

            log::trace!("found match by type: {id} : {}", argument_value.ty());
            match self.insert_and_remove_from_parameters(
                &id,
                argument_value.value.clone(),
                &parameter_value,
                parameter_values,
            )? {
                TypeCheckResult::MultiMatch | TypeCheckResult::SingleMatch => (),
                _ => {
                    if idx == 0 {
                        break;
                    }
                    idx -= 1;
                }
            }
        }

        Ok(())
    }

    /// Find default parameters and insert them into the map.
    ///
    /// If a parameter has a default value and is not present in the arguments, insert the default value.
    /// The parameter is then removed from the list of parameters.
    fn match_defaults(
        &mut self,
        argument_values: &ArgumentValueList,
        parameter_values: &mut ParameterValueList,
    ) -> EvalResult<()> {
        if parameter_values.is_empty() {
            return Ok(());
        }
        parameter_values
            // Clone the list of parameters because we want to remove elements from it while iterating
            .clone()
            .iter()
            // Filter out parameters that have a default value and are not present in the arguments
            .filter_map(
                |(id, p)| match (argument_values.get(id), &p.default_value) {
                    (None, Some(default_value)) => Some((id, p, default_value.clone())),
                    _ => None,
                },
            )
            // Insert the default values into the map
            .try_for_each(|(id, parameter_value, default_value)| {
                log::trace!("found match by default: {id} = {default_value}");
                self.insert_and_remove_from_parameters(
                    id,
                    default_value,
                    parameter_value,
                    parameter_values,
                )?;
                Ok(())
            })
    }

    /// Find a match between arguments and parameters.
    ///
    /// This function tries to find a match between the arguments and the parameters.
    fn find_match(
        argument_values: &ArgumentValueList,
        parameter_values: &ParameterValueList,
    ) -> EvalResult<Self> {
        log::trace!(
            "find match:\n   Arguments: {argument_values}\n  Parameters: {parameter_values}"
        );

        let mut missing_parameter_values = parameter_values.clone();
        let mut result = Self::new(argument_values.src_ref());

        result.match_named(argument_values, &mut missing_parameter_values)?;
        result.match_unnamed(argument_values, &mut missing_parameter_values)?;
        result.match_defaults(argument_values, &mut missing_parameter_values)?;

        if !missing_parameter_values.is_empty() {
            return Err(EvalError::MissingArguments(missing_parameter_values));
        }

        log::trace!("found match:\n   Arguments: {result}");
        Ok(result)
    }
}

#[test]
fn argument_match_single() {
    // create parameters and arguments to test
    let parameters: ParameterValueList = [crate::parameter!(a: Scalar)].into_iter().collect();
    let arguments: ArgumentValueList = [crate::argument!(a: Scalar = 5.0)].into_iter().collect();

    // test find_match
    let arg_map = ArgumentMap::find_match(&arguments, &parameters).expect("Valid match");

    // check output
    assert_eq!(
        arg_map
            .get_value(&Identifier::no_ref("a"))
            .expect("internal test error"),
        &Value::Quantity(Quantity::new(5.0, QuantityType::Scalar))
    );
}

#[test]
fn argument_match_unnamed() {
    // create parameters and arguments to test
    let parameters: ParameterValueList = [
        crate::parameter!(a: Scalar),
        crate::parameter!(b: Scalar),
        crate::parameter!(c: Length),
    ]
    .into_iter()
    .collect();
    let arguments: ArgumentValueList = [
        crate::argument!(Scalar = 1.0),
        crate::argument!(Length = 3.0),
        crate::argument!(b: Scalar = 2.0),
    ]
    .into_iter()
    .collect();

    // test find_match
    let arg_map = ArgumentMap::find_match(&arguments, &parameters).expect("Valid match");

    // check output
    assert_eq!(
        arg_map
            .get_value(&Identifier::no_ref("a"))
            .expect("internal test error"),
        &Value::Quantity(Quantity::new(1.0, QuantityType::Scalar))
    );
    assert_eq!(
        arg_map
            .get_value(&Identifier::no_ref("b"))
            .expect("internal test error"),
        &Value::Quantity(Quantity::new(2.0, QuantityType::Scalar))
    );
    assert_eq!(
        arg_map
            .get_value(&Identifier::no_ref("c"))
            .expect("internal test error"),
        &Value::Quantity(Quantity::new(3.0, QuantityType::Scalar))
    );
}
