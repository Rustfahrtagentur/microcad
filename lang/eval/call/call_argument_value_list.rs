// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Call argument value list evaluation entity

use crate::{eval::*, ord_map::*, src_ref::*};

/// List of call argument values
#[derive(Clone, Debug, Default)]
pub struct CallArgumentValueList(Refer<OrdMap<Id, CallArgumentValue>>);

impl CallArgumentValueList {
    /// Insert into the argument map and remove the parameter from the list of parameters
    fn insert_and_remove(
        arg_map: &mut ArgumentMap,
        parameter_values: &mut ParameterValueList,
        name: &Id,
        value: Value,
    ) {
        arg_map.insert(name.clone(), value.clone());
        parameter_values.remove(name);
    }

    fn insert_into_multi_arg_map(
        arg_map: &mut MultiArgumentMap,
        parameter_value: ParameterValue,
        parameter_values: &mut ParameterValueList,
        value: Value,
    ) -> TypeCheckResult {
        let result = parameter_value.type_check(&value.ty());
        match result {
            TypeCheckResult::MultiMatch => match &value {
                Value::List(l) => {
                    parameter_values.remove(&parameter_value.name);
                    arg_map.insert_multi(parameter_value.name.clone(), l.fetch());
                    result
                }
                value => panic!("Expected list type, got {}", value.ty()),
            },
            TypeCheckResult::SingleMatch => {
                parameter_values.remove(&parameter_value.name);
                arg_map.insert_single(parameter_value.name.clone(), value);
                result
            }
            _ => result,
        }
    }

    fn get_multi_matching_named_arguments(
        &self,
        parameter_values: &mut ParameterValueList,
        arg_map: &mut MultiArgumentMap,
    ) -> Result<()> {
        let old_parameter_values = parameter_values.clone();

        // Iterate over defined parameters and check if the call arguments contains an argument with the same as the parameter
        old_parameter_values.iter().for_each(|parameter_value| {
            match self.get(&parameter_value.name) {
                // We have a matching argument with the same name as the parameter.
                Some(arg) => {
                    // Now we need to check if the argument type matches the parameter type
                    Self::insert_into_multi_arg_map(
                        arg_map,
                        parameter_value.clone(),
                        parameter_values,
                        arg.value.clone(),
                    );
                }
                // No matching argument found, check if a default value is defined
                None => {
                    // If we have a default value, we keep in the map and use it later via `get_multi_insert_default_parameters`
                    /*if let Some(default) = &parameter_value.default_value {
                        Self::insert_into_multi_arg_map(
                            arg_map,
                            parameter_value.clone(),
                            parameter_values,
                            default.clone(),
                        );
                    }*/
                }
            }
        });

        Ok(())
    }

    fn get_multi_insert_default_parameters(
        &self,
        parameter_values: &mut ParameterValueList,
        arg_map: &mut MultiArgumentMap,
    ) -> Result<()> {
        let old_parameter_values = parameter_values.clone();

        // Iterate over defined parameters and check if the call arguments contains an argument with the same as the parameter
        old_parameter_values.iter().for_each(|parameter_value| {
            if self.get(&parameter_value.name).is_none() {
                // If we have a default value, we can use it
                if let Some(default) = &parameter_value.default_value {
                    Self::insert_into_multi_arg_map(
                        arg_map,
                        parameter_value.clone(),
                        parameter_values,
                        default.clone(),
                    );
                }
            }
        });

        Ok(())
    }

    fn get_matching_named_arguments(
        &self,
        parameter_values: &mut ParameterValueList,
        arg_map: &mut ArgumentMap,
    ) -> Result<()> {
        let old_parameter_values = parameter_values.clone();

        // Iterate over defined parameters and check if the call arguments contains an argument with the same as the parameter
        old_parameter_values.iter().for_each(|parameter_value| {
            match self.get(&parameter_value.name) {
                // We have a matching argument with the same name as the parameter.
                Some(arg) => {
                    // Now we need to check if the argument type matches the parameter type
                    if let TypeCheckResult::SingleMatch =
                        parameter_value.type_check(&arg.value.ty())
                    {
                        Self::insert_and_remove(
                            arg_map,
                            parameter_values,
                            &parameter_value.name,
                            arg.value.clone(),
                        );
                    }
                }
                // No matching argument found, check if a default value is defined
                None => {
                    // If we have a default value, we can use it
                    if let Some(default) = &parameter_value.default_value {
                        Self::insert_and_remove(
                            arg_map,
                            parameter_values,
                            &parameter_value.name,
                            default.clone(),
                        );
                    }
                }
            }
        });

        Ok(())
    }

    fn get_matching_positional_arguments(
        &self,
        parameter_values: &mut ParameterValueList,
        arg_map: &mut ArgumentMap,
    ) -> Result<()> {
        if parameter_values.is_empty() {
            return Ok(());
        }
        let mut positional_index = 0;

        self.iter()
            .filter(|arg| arg.name.is_none())
            .try_for_each(|arg| {
                use std::ops::ControlFlow;

                match parameter_values.get_by_index(positional_index) {
                    Some(param_value) => {
                        let param_name = param_value.name.clone();
                        if !arg_map.contains_key(&param_name) {
                            if let TypeCheckResult::SingleMatch =
                                param_value.type_check(&arg.value.ty())
                            {
                                Self::insert_and_remove(
                                    arg_map,
                                    parameter_values,
                                    &param_name,
                                    arg.value.clone(),
                                );
                                if positional_index >= parameter_values.len() {
                                    return ControlFlow::Break(());
                                }
                            }
                        } else {
                            positional_index += 1;
                        }
                    }
                    None => {
                        return ControlFlow::Break(());
                    }
                }

                ControlFlow::Continue(())
            });

        Ok(())
    }

    fn get_multi_positional_arguments(
        &self,
        parameter_values: &mut ParameterValueList,
        arg_map: &mut MultiArgumentMap,
    ) -> Result<()> {
        if parameter_values.is_empty() {
            return Ok(());
        }
        let mut positional_index = 0;

        self.iter()
            .filter(|arg| arg.name.is_none())
            .try_for_each(|arg| {
                use std::ops::ControlFlow;

                match parameter_values.get_by_index(positional_index) {
                    Some(param_value) => {
                        let param_name = param_value.name.clone();
                        if !arg_map.contains_key(&param_name) {
                            match Self::insert_into_multi_arg_map(
                                arg_map,
                                param_value.clone(),
                                parameter_values,
                                arg.value.clone(),
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
                    None => ControlFlow::Break(()),
                }
            });

        Ok(())
    }

    /// This functions checks if the call arguments match the given parameter definitions
    /// It returns a map of arguments that match the parameters
    pub fn get_matching_arguments(
        &self,
        parameter_values: &ParameterValueList,
    ) -> Result<ArgumentMap> {
        let mut arg_map = ArgumentMap::new(self.src_ref());

        // Check for unexpected arguments.
        // We are looking for call arguments that are not in the parameter list
        if let Some(name) = self
            .keys()
            .find(|name| parameter_values.get_by_name(name).is_none())
        {
            return Err(EvalError::UnexpectedArgument(name.clone()));
        }

        let mut missing_parameter_values = parameter_values.clone();

        self.get_matching_named_arguments(&mut missing_parameter_values, &mut arg_map)?;
        self.get_matching_positional_arguments(&mut missing_parameter_values, &mut arg_map)?;

        if !missing_parameter_values.is_empty() {
            return Err(EvalError::MissingArguments(
                missing_parameter_values
                    .iter()
                    .map(|parameter| parameter.name.clone())
                    .collect::<Vec<_>>(),
            ));
        }

        Ok(arg_map)
    }

    /// Get multiplicity of matching arguments
    pub fn get_multi_matching_arguments(
        &self,
        parameter_values: &ParameterValueList,
    ) -> Result<MultiArgumentMap> {
        let mut multi_arg_map = MultiArgumentMap::default();

        // Check for unexpected arguments.
        // We are looking for call arguments that are not in the parameter list
        if let Some(name) = self
            .keys()
            .find(|name| parameter_values.get_by_name(name).is_none())
        {
            return Err(EvalError::UnexpectedArgument(name.clone()));
        }

        let mut missing_parameter_values = parameter_values.clone();

        self.get_multi_matching_named_arguments(&mut missing_parameter_values, &mut multi_arg_map)?;
        self.get_multi_positional_arguments(&mut missing_parameter_values, &mut multi_arg_map)?;
        self.get_multi_insert_default_parameters(
            &mut missing_parameter_values,
            &mut multi_arg_map,
        )?;

        if !missing_parameter_values.is_empty() {
            return Err(EvalError::MissingArguments(
                missing_parameter_values
                    .iter()
                    .map(|parameter| parameter.name.clone())
                    .collect::<Vec<_>>(),
            ));
        }

        Ok(multi_arg_map)
    }
}

impl SrcReferrer for CallArgumentValueList {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

impl std::ops::Deref for CallArgumentValueList {
    type Target = OrdMap<Id, CallArgumentValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for CallArgumentValueList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
impl From<Vec<CallArgumentValue>> for CallArgumentValueList {
    fn from(value: Vec<CallArgumentValue>) -> Self {
        let src_ref = SrcRef::from_vec(&value);
        Self(Refer::new(value.into(), src_ref))
    }
}

#[cfg(test)]
macro_rules! assert_eq_arg_map_value {
    ($arg_map:ident, $($name:ident: $ty:ident = $value:expr),*) => {
        $(assert_eq!(
            $arg_map.get(stringify!($name)).unwrap(),
            &Value::$ty(crate::src_ref::Refer::none($value))
        ));*
    };
}

#[test]
fn call_get_matching_arguments() {
    use crate::{parameter_value, r#type::*};

    // module my_module(foo: Integer, bar: Integer, baz: Scalar = 4.0)
    let param_values = ParameterValueList::new(vec![
        parameter_value!(foo: Integer),
        parameter_value!(bar: Integer),
        parameter_value!(baz: Scalar = 4.0),
    ]);

    // my_module(1, bar = 2, baz = 3.0)
    let call_values = CallArgumentValueList::from(vec![
        call_argument_value!(Integer = 1),
        call_argument_value!(foo: Integer = 2),
        call_argument_value!(baz: Scalar = 3.0),
    ]);

    let arg_map = call_values.get_matching_arguments(&param_values).unwrap();

    assert_eq_arg_map_value!(arg_map,
        foo: Integer = 2,
        bar: Integer = 1,
        baz: Scalar = 3.0
    );
}

#[test]
fn call_get_matching_arguments_missing() {
    use crate::{parameter_value, r#type::*};

    let param_values = ParameterValueList::new(vec![
        parameter_value!(foo: Integer),
        parameter_value!(bar: Integer),
        parameter_value!(baz: Scalar = 4.0),
    ]);
    let call_values = CallArgumentValueList::from(vec![
        call_argument_value!(Integer = 1),
        call_argument_value!(baz: Scalar = 3.0),
    ]);

    let arg_map = call_values.get_matching_arguments(&param_values);

    if let Err(EvalError::MissingArguments(missing)) = arg_map {
        assert_eq!(missing.len(), 1);
        assert_eq!(&missing[0], "bar");
    } else {
        panic!("Expected MissingArguments error");
    }
}

#[test]
fn get_multi_matching_arguments() {
    use crate::{parameter_value, r#type::*};

    let param_values = ParameterValueList::new(vec![
        parameter_value!(thickness: Scalar = 2.0),
        parameter_value!(inner_diameter: Scalar = 100.0),
        parameter_value!(height: Scalar = 10.0),
    ]);

    let call_values = CallArgumentValueList::from(vec![
        call_argument_value!(Scalar = 2.0),
        call_argument_value!(Scalar = 100.0),
        call_argument_value!(Scalar = 10.0),
    ]);

    let multi_argument_map = call_values
        .get_multi_matching_arguments(&param_values)
        .expect("MultiArgumentMap expected");

    for argument_map in multi_argument_map.combinations() {
        assert_eq_arg_map_value!(argument_map,
            thickness: Scalar = 2.0,
            inner_diameter: Scalar = 100.0,
            height: Scalar = 10.0
        );
    }
}
