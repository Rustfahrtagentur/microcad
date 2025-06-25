// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! *Argument value list* evaluation entity.

use crate::{eval::*, ord_map::*, src_ref::*, value::*};

/// Collection of *argument values* (e.g. `( x=1, y=2 )`).
///
/// Also provides methods to find a matching call
/// between it and a given *parameter list*.
#[derive(Clone, Debug, Default)]
pub struct ArgumentValueList(Refer<OrdMap<Identifier, ArgumentValue>>);

impl ArgumentValueList {
    /// Create a *argument value list*.
    ///
    /// Transports code into builtin in `impl` [`Eval`] for [`Call`].
    ///
    /// Shall only be used for builtin symbols.
    /// # Arguments
    pub fn from_code(code: String, referrer: impl SrcReferrer) -> Self {
        let mut value = OrdMap::default();
        value
            .try_push(ArgumentValue::new(
                None,
                Value::String(code),
                referrer.src_ref(),
            ))
            .expect("map with one element");
        Self(Refer {
            value,
            src_ref: referrer.src_ref(),
        })
    }

    /// Return a single argument.
    ///
    /// Returns error if there is no or more than one argument available.
    pub fn get_single(&self) -> EvalResult<&ArgumentValue> {
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

    /// Check for unexpected arguments.
    ///
    /// This method will return an error if there is a argument that
    /// is not in the parameter list.
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

    /// This functions checks if the arguments match the given parameter definitions.
    ///
    /// Returns a map of arguments that match the parameters.
    pub fn get_matching_arguments(
        &self,
        context: &mut Context,
        parameters: &ParameterList,
    ) -> EvalResult<ArgumentMap> {
        let parameters = parameters.eval(context)?;
        ArgumentMap::find_match(self, &parameters)
    }

    /// Get multiplicity of matching arguments.
    pub fn get_multi_matching_arguments(
        &self,
        context: &mut Context,
        parameters: &ParameterList,
    ) -> EvalResult<MultiArgumentMap> {
        let parameters = parameters.eval(context)?;
        MultiArgumentMap::find_match(self, &parameters)
    }
}

impl SrcReferrer for ArgumentValueList {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

impl std::ops::Deref for ArgumentValueList {
    type Target = OrdMap<Identifier, ArgumentValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ArgumentValueList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for ArgumentValueList {
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
impl From<Vec<ArgumentValue>> for ArgumentValueList {
    fn from(value: Vec<ArgumentValue>) -> Self {
        Self(Refer::none(value.into()))
    }
}

#[cfg(test)]
macro_rules! assert_eq_arg_map_value {
    ($arg_map:ident, $($name:ident: $ty:ident = $value:expr),*) => {
        $(assert_eq!(
            $arg_map.get(&Identifier::no_ref(stringify!($name).into())).expect(&format!("Argument `{}` expected",stringify!($name))),
            &crate::builtin::BuiltinValueHelper::$ty($value).into()
        ));*
    };
}

#[test]
fn call_get_matching_arguments() {
    use crate::{argument, parameter};

    // my_part(foo: Integer, bar: Integer, baz: Scalar = 4.0)
    let param_values = vec![
        parameter!(foo: Integer),
        parameter!(bar: Integer),
        parameter!(baz: Scalar = 4.0),
    ]
    .into();

    // my_part(1, bar = 2, baz = 3.0)
    let call_values = ArgumentValueList::from(vec![
        argument!(Integer = 1),
        argument!(foo: Integer = 2),
        argument!(baz: Scalar = 3.0),
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
    use crate::{argument, parameter};

    // function f(foo: Integer, bar: Integer, baz: Scalar = 4.0)
    let parameters = ParameterValueList::new(vec![
        parameter!(foo: Integer),
        parameter!(bar: Integer),
        parameter!(baz: Scalar = 4.0),
    ]);

    // f(1, baz = 3.0)
    let arguments =
        ArgumentValueList::from(vec![argument!(Integer = 1), argument!(baz: Scalar = 3.0)]);

    let arg_map = ArgumentMap::find_match(&arguments, &parameters);

    if let Err(EvalError::MissingArguments(missing)) = arg_map {
        assert_eq!(missing.len(), 1);
        assert_eq!(&missing[0].id, "bar");
    } else {
        panic!("Expected MissingArguments error");
    }
}

#[test]
fn get_multi_matching_arguments() {
    use crate::{argument, parameter};

    let parameters = ParameterValueList::new(vec![
        parameter!(thickness: Scalar = 2.0),
        parameter!(inner_diameter: Scalar = 100.0),
        parameter!(height: Scalar = 10.0),
    ]);

    let arguments = ArgumentValueList::from(vec![
        argument!(Scalar = 2.0),
        argument!(Scalar = 100.0),
        argument!(Scalar = 10.0),
    ]);

    let multi_argument_map =
        MultiArgumentMap::find_match(&arguments, &parameters).expect("Valid match");

    for argument_map in multi_argument_map.combinations() {
        assert_eq_arg_map_value!(argument_map,
            thickness: Scalar = 2.0,
            inner_diameter: Scalar = 100.0,
            height: Scalar = 10.0
        );
    }
}
