use crate::language::lang_type::Ty;

use super::{
    ArgumentMap, CallArgumentValue, Error, Identifier, IdentifierList, ParameterValueList,
    TypeCheckResult, Value,
};

#[derive(Clone, Debug, Default)]
pub struct CallArgumentValueList {
    arguments: Vec<CallArgumentValue>,
    named: std::collections::HashMap<Identifier, usize>,
}

impl CallArgumentValueList {
    pub fn new(args: Vec<CallArgumentValue>) -> Self {
        // TODO: prevent mut
        let mut l = Self::default();
        for arg in args {
            l.push(arg);
        }
        l
    }

    pub fn get_by_name(&self, name: &Identifier) -> Option<&CallArgumentValue> {
        self.named.get(name).map(|index| &self.arguments[*index])
    }

    pub fn push(&mut self, arg: CallArgumentValue) {
        self.arguments.push(arg.clone());
        if let Some(name) = arg.name {
            self.named.insert(name.clone(), self.arguments.len() - 1);
        }
    }

    /// Insert into the argument map and remove the parameter from the list of parameters
    fn insert_and_remove(
        arg_map: &mut ArgumentMap,
        parameter_values: &mut ParameterValueList,
        name: &Identifier,
        value: Value,
    ) {
        arg_map.insert(name.clone(), value.clone());
        parameter_values.remove(name);
    }

    fn get_matching_named_arguments(
        &self,
        parameter_values: &mut ParameterValueList,
        arg_map: &mut ArgumentMap,
    ) -> Result<(), Error> {
        let old_parameter_values = parameter_values.clone();

        // Iterate over defined parameters and check if the call arguments contains an argument with the same as the parameter
        old_parameter_values.iter().for_each(|parameter_value| {
            match self.get_by_name(parameter_value.name()) {
                // We have a matching argument with the same name as the parameter.
                Some(arg) => {
                    // Now we need to check if the argument type matches the parameter type
                    if let TypeCheckResult::Ok = parameter_value.type_check(&arg.value.ty()) {
                        Self::insert_and_remove(
                            arg_map,
                            parameter_values,
                            parameter_value.name(),
                            arg.value.clone(),
                        );
                    }
                }
                // No matching argument found, check if a default value is defined
                None => {
                    // If we have a default value, we can use it
                    if let Some(default) = parameter_value.default_value() {
                        Self::insert_and_remove(
                            arg_map,
                            parameter_values,
                            parameter_value.name(),
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
    ) -> Result<(), Error> {
        if parameter_values.is_empty() {
            return Ok(());
        }
        let mut positional_index = 0;
        for arg in &self.arguments {
            if arg.name.is_none() {
                let param_value = parameter_values[positional_index].clone();
                if !arg_map.contains_key(param_value.name()) {
                    // @todo: Check for tuple arguments and whether the tuple fields match the parameters
                    if let TypeCheckResult::Ok =
                        parameter_values[positional_index].type_check(&arg.value.ty())
                    {
                        Self::insert_and_remove(
                            arg_map,
                            parameter_values,
                            param_value.name(),
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
    ) -> Result<ArgumentMap, Error> {
        let mut arg_map = ArgumentMap::new();

        // Check for unexpected arguments.
        // We are looking for call arguments that are not in the parameter list
        for name in self.named.keys() {
            if parameter_values.get(name).is_none() {
                return Err(Error::UnexpectedArgument(name.clone()));
            }
        }

        let mut missing_parameter_values = parameter_values.clone();

        self.get_matching_named_arguments(&mut missing_parameter_values, &mut arg_map)?;
        self.get_matching_positional_arguments(&mut missing_parameter_values, &mut arg_map)?;

        if !missing_parameter_values.is_empty() {
            // TODO: prevent mut and for
            let mut missing_args = IdentifierList::new();
            for parameter in missing_parameter_values.iter() {
                missing_args.push(parameter.name().clone()).unwrap(); // Unwrap is safe here because we know the parameter is unique
            }
            return Err(Error::MissingArguments(missing_args));
        }

        Ok(arg_map)
    }
}

#[cfg(test)]
macro_rules! assert_eq_arg_map_value {
    ($arg_map:ident, $($name:ident: $ty:ident = $value:expr),*) => {
        $(assert_eq!(
            $arg_map.get(&stringify!($name).into()).unwrap(),
            &Value::$ty($value)
        ));*
    };
}

#[test]
fn call_get_matching_arguments() {
    use crate::{language::lang_type::Type, parameter_value};

    // module my_module(foo: Integer, bar: Integer, baz: Scalar = 4.0)
    let param_values = ParameterValueList::new(vec![
        parameter_value!(foo: Integer),
        parameter_value!(bar: Integer),
        parameter_value!(baz: Scalar = 4.0),
    ]);

    // my_module(1, bar = 2, baz = 3.0)
    let call_values = CallArgumentValueList::new(vec![
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
    use crate::{language::lang_type::Type, parameter_value};

    let param_values = ParameterValueList::new(vec![
        parameter_value!(foo: Integer),
        parameter_value!(bar: Integer),
        parameter_value!(baz: Scalar = 4.0),
    ]);
    let call_values = CallArgumentValueList::new(vec![
        call_argument_value!(Integer = 1),
        call_argument_value!(baz: Scalar = 3.0),
    ]);

    let arg_map = call_values.get_matching_arguments(&param_values);

    if let Err(Error::MissingArguments(missing)) = arg_map {
        assert_eq!(missing.len(), 1);
        assert_eq!(&missing[0], "bar");
    } else {
        panic!("Expected MissingArguments error");
    }
}

/*
#[test]
fn tuple_substitution() {
    let param_values = ParameterValueList::new(vec![
        parameter_value!(foo: Integer),
        parameter_value!(bar: Integer),
        parameter_value!(baz: Scalar = 4.0),
    ]);
    let call_values = CallArgumentValueList::new(vec![
        call_argument_value!(NamedTuple = named_tuple!(foo: Integer = 1, bar: Integer = 2)),
        call_argument_value!(baz: Scalar = 3.0),
    ]);

    let arg_map = call_values.get_matching_arguments(&param_values).unwrap();
    assert_eq_arg_map_value!(arg_map, foo: Integer = 1, bar: Integer = 2, baz: Scalar = 3.0);
}
*/
