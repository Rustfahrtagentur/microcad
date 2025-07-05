// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! *Argument value list* evaluation entity.

use crate::{eval::*, src_ref::*, value::*};

/// Collection of *argument values* (e.g. `( x=1, y=2 )`).
///
/// Also provides methods to find a matching call
/// between it and a given *parameter list*.
#[derive(Clone, Debug, Default)]
pub struct ArgumentValueList {
    map: std::collections::HashMap<Identifier, ArgumentValue>,
    src_ref: SrcRef,
}

impl ArgumentValueList {
    /// Create a *argument value list*.
    ///
    /// Transports code into builtin in `impl` [`Eval`] for [`Call`].
    ///
    /// Shall only be used for builtin symbols.
    /// # Arguments
    pub fn from_code(code: String, referrer: impl SrcReferrer) -> Self {
        let map: std::collections::HashMap<Identifier, ArgumentValue> = [(
            Identifier::none(),
            (ArgumentValue::new(Value::String(code), referrer.src_ref())),
        )]
        .into_iter()
        .collect();
        Self {
            map,
            src_ref: referrer.src_ref(),
        }
    }

    /// Return a single argument.
    ///
    /// Returns error if there is no or more than one argument available.
    pub fn get_single(&self) -> EvalResult<(&Identifier, &ArgumentValue)> {
        if self.map.len() == 1 {
            if let Some(a) = self.map.iter().next() {
                return Ok(a);
            }
        }

        Err(EvalError::ArgumentCountMismatch {
            args: self.clone(),
            expected: 1,
            found: self.map.len(),
        })
    }

    pub fn get_by_type(&self, ty: &Type) -> Option<(&Identifier, &ArgumentValue)> {
        self.map.iter().find(|(_, arg)| arg.value.ty() == *ty)
    }
    /// Check for unexpected arguments.
    ///
    /// This method will return an error if there is an argument that
    /// is not in the parameter list.
    pub fn check_for_unexpected_arguments(
        &self,
        parameter_values: &ParameterValueList,
    ) -> EvalResult<()> {
        log::trace!("check_for_unexpected_arguments:\n{parameter_values:#?}\n------\n{self:#?}");
        self.map.iter().try_for_each(|(id, arg)| {
            if !id.is_empty() && parameter_values.get(id).is_some() {
                return Ok(());
            }
            if parameter_values.get_by_type(arg.value.ty()).is_ok() {
                return Ok(());
            }
            Err(EvalError::UnexpectedArgument(id.clone()))
        })
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
        ArgumentMatch::find_match(self, &parameters)
    }

    /// Get multiplicity of matching arguments.
    pub fn get_multi_matching_arguments(
        &self,
        context: &mut Context,
        parameters: &ParameterList,
    ) -> EvalResult<MultiArgumentMap> {
        let parameters = parameters.eval(context)?;
        todo!()
        //MultiArgumentMap::find_match(self, &parameters)
    }

    pub fn single_unnamed(&self) -> EvalResult<&ArgumentValue> {
        let v: Vec<_> = self.iter().filter(|(id, _)| id.is_empty()).collect();
        let len = v.len();
        if len != 1 {
            Err(EvalError::ArgumentCountMismatch {
                args: self.clone(),
                expected: 1,
                found: len,
            })
        } else {
            Ok(v.first().expect("existing argument").1)
        }
    }
}

impl std::ops::Deref for ArgumentValueList {
    type Target = std::collections::HashMap<Identifier, ArgumentValue>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl SrcReferrer for ArgumentValueList {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for ArgumentValueList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.map
                .values()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl<I, A> FromIterator<(I, A)> for ArgumentValueList
where
    I: Into<Identifier>,
    A: Into<ArgumentValue>,
{
    fn from_iter<T: IntoIterator<Item = (I, A)>>(iter: T) -> Self {
        Self {
            map: iter
                .into_iter()
                .map(|(i, a)| (i.into(), a.into()))
                .collect(),
            src_ref: SrcRef(None),
        }
    }
}
