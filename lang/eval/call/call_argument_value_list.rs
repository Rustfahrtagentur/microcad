// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Call argument value list evaluation entity

use crate::{eval::*, ord_map::*, src_ref::*, value::*};

/// List of call argument values
///
/// This class also provides methods to find a matching call
/// between between the call argument value list and a given parameter list.
#[derive(Clone, Debug, Default)]
pub struct CallArgumentValueList(Refer<OrdMap<Identifier, CallArgumentValue>>);

impl CallArgumentValueList {
    /// return a single argument
    pub fn get_single(&self) -> EvalResult<&CallArgumentValue> {
        if self.len() == 1 {
            if let Some(a) = self.0.first() {
                return Ok(a);
            }
        }

        Err(EvalError::ArgumentCountMismatch {
            args: self.clone(),
            expected: 1,
            found: self.len(),
        })
    }

    /// Create a `CallArgumentValueList` from a `CallArgumentList`
    pub fn from_call_argument_list(
        call_argument_list: &CallArgumentList,
        context: &mut Context,
    ) -> EvalResult<Self> {
        let mut v = CallArgumentValueList::default();
        for call_arg in call_argument_list.iter() {
            v.push(call_arg.eval_value(context)?)
                .expect("Could not insert call argument value");
        }
        Ok(v)
    }

    /// Check for unexpected arguments.
    ///
    /// This method will return an error if there is a call argument that is not in the parameter list
    pub fn check_for_unexpected_arguments(
        &self,
        parameter_values: &ParameterValueList,
    ) -> EvalResult<()> {
        match self
            .keys()
            .find(|id| parameter_values.get_by_id(id).is_none())
        {
            Some(id) => Err(EvalError::UnexpectedArgument(id.clone())),
            None => Ok(()),
        }
    }

    /// This functions checks if the call arguments match the given parameter definitions
    /// It returns a map of arguments that match the parameters
    pub fn get_matching_arguments(
        &self,
        context: &mut Context,
        parameters: &ParameterList,
    ) -> EvalResult<ArgumentMap> {
        let parameters = ParameterValueList::from_parameter_list(parameters, context)?;
        ArgumentMap::find_match(self, &parameters)
    }

    /// Get multiplicity of matching arguments
    pub fn get_multi_matching_arguments(
        &self,
        context: &mut Context,
        parameters: &ParameterList,
    ) -> EvalResult<MultiArgumentMap> {
        let parameters = ParameterValueList::from_parameter_list(parameters, context)?;
        MultiArgumentMap::find_match(self, &parameters)
    }
}

impl SrcReferrer for CallArgumentValueList {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

impl std::ops::Deref for CallArgumentValueList {
    type Target = OrdMap<Identifier, CallArgumentValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for CallArgumentValueList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for CallArgumentValueList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .value
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
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
            $arg_map.get(&Identifier::no_ref(stringify!($name).into())).expect(&format!("Argument `{}` expected",stringify!($name))),
            &Value::$ty(crate::src_ref::Refer::none($value))
        ));*
    };
}

#[test]
fn call_get_matching_arguments() {
    use crate::{parameter_value, ty::*};

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

    let arg_map = ArgumentMap::find_match(&call_values, &param_values).expect("Valid match");

    assert_eq_arg_map_value!(arg_map,
        foo: Integer = 2,
        bar: Integer = 1,
        baz: Scalar = 3.0
    );
}

#[test]
fn call_get_matching_arguments_missing() {
    use crate::{parameter_value, ty::*};

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

    let arg_map = ArgumentMap::find_match(&call_values, &param_values);

    if let Err(EvalError::ValueError(ValueError::MissingArguments(missing))) = arg_map {
        assert_eq!(missing.len(), 1);
        assert_eq!(&missing[0].id, "bar");
    } else {
        panic!("Expected MissingArguments error");
    }
}

#[test]
fn get_multi_matching_arguments() {
    use crate::{parameter_value, ty::*};

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

    let multi_argument_map =
        MultiArgumentMap::find_match(&call_values, &param_values).expect("Valid match");

    for argument_map in multi_argument_map.combinations() {
        assert_eq_arg_map_value!(argument_map,
            thickness: Scalar = 2.0,
            inner_diameter: Scalar = 100.0,
            height: Scalar = 10.0
        );
    }
}
