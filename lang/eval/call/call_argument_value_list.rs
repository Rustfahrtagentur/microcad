// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Call argument value list evaluation entity

use crate::{eval::*, ord_map::*, src_ref::*, *};

/// List of call argument values, (foo = 4.0, 3.0mm, bar)
///
/// This class also provides methods to find a matching call
/// between between the call argument value list and a given parameter list.
#[derive(Clone, Debug, Default)]
pub struct CallArgumentValueList(Refer<OrdMap<Id, CallArgumentValue>>);

impl CallArgumentValueList {
    /// Check for unexpected arguments.
    ///
    /// This method will return an error if there is a call argument that is not in the parameter list
    pub fn check_for_unexpected_arguments(
        &self,
        parameter_values: &ParameterValueList,
    ) -> EvalResult<()> {
        match self
            .keys()
            .find(|name| parameter_values.get_by_name(name).is_none())
        {
            Some(name) => Err(EvalError::UnexpectedArgument(name.clone())),
            None => Ok(()),
        }
    }

    /// This functions checks if the call arguments match the given parameter definitions
    /// It returns a map of arguments that match the parameters
    pub fn get_matching_arguments(
        &self,
        parameter_values: &ParameterValueList,
    ) -> EvalResult<ArgumentMap> {
        ArgumentMap::find_match(self, parameter_values)
    }

    /// Get multiplicity of matching arguments
    pub fn get_multi_matching_arguments(
        &self,
        parameter_values: &ParameterValueList,
    ) -> EvalResult<MultiArgumentMap> {
        use ArgumentMatch;
        MultiArgumentMap::find_match(self, parameter_values)
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
            $arg_map.get(stringify!($name)).expect(&format!("Argument `{}` expected",stringify!($name))),
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

    let arg_map = call_values
        .get_matching_arguments(&param_values)
        .expect("test error");

    assert_eq_arg_map_value!(arg_map,
        foo: Integer = 2,
        bar: Integer = 1,
        baz: Scalar = 3.0
    );
}

#[test]
fn call_get_matching_arguments_missing() {
    use crate::{parameter_value, r#type::*};

    // function f(foo: Integer, bar: Integer, baz: Scalar = 4.0)
    let param_values = ParameterValueList::new(vec![
        parameter_value!(foo: Integer),
        parameter_value!(bar: Integer),
        parameter_value!(baz: Scalar = 4.0),
    ]);

    // f(1, baz = 3.0)
    let call_values = CallArgumentValueList::from(vec![
        call_argument_value!(Integer = 1),
        call_argument_value!(baz: Scalar = 3.0),
    ]);

    let arg_map = call_values.get_matching_arguments(&param_values);

    if let Err(EvalError::ValueError(ValueError::MissingArguments(missing))) = arg_map {
        assert_eq!(missing.len(), 1);
        assert_eq!(&missing[0].name, "bar");
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
