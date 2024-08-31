use crate::{eval::*, ord_map::OrdMap};

#[derive(Clone, Debug, Default)]
pub struct CallArgumentValueList(OrdMap<Id, CallArgumentValue>);

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

impl From<Vec<CallArgumentValue>> for CallArgumentValueList {
    fn from(value: Vec<CallArgumentValue>) -> Self {
        Self(OrdMap::<Id, CallArgumentValue>::from(value))
    }
}

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
                    if let TypeCheckResult::Ok = parameter_value.type_check(&arg.value.ty()) {
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
        for arg in self.iter() {
            if arg.name.is_none() {
                let param_value = parameter_values[positional_index].clone();
                if !arg_map.contains_key(&param_value.name) {
                    // @todo: Check for tuple arguments and whether the tuple fields match the parameters
                    if let TypeCheckResult::Ok =
                        parameter_values[positional_index].type_check(&arg.value.ty())
                    {
                        Self::insert_and_remove(
                            arg_map,
                            parameter_values,
                            &param_value.name,
                            arg.value.clone(),
                        );
                        if positional_index >= parameter_values.len() {
                            break;
                        }
                    }
                } else {
                    positional_index += 1;
                }
            }
        }

        Ok(())
    }

    /// This functions checks if the call arguments match the given parameter definitions
    /// It returns a map of arguments that match the parameters
    pub fn get_matching_arguments(
        &self,
        parameter_values: &ParameterValueList,
    ) -> Result<ArgumentMap> {
        let mut arg_map = ArgumentMap::new();

        // Check for unexpected arguments.
        // We are looking for call arguments that are not in the parameter list
        for name in self.keys() {
            if parameter_values.get(name).is_none() {
                return Err(EvalError::UnexpectedArgument(name.clone()));
            }
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
}

#[cfg(test)]
macro_rules! assert_eq_arg_map_value {
    ($arg_map:ident, $($name:ident: $ty:ident = $value:expr),*) => {
        $(assert_eq!(
            $arg_map.get(stringify!($name)).unwrap(),
            &Value::$ty($value)
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
