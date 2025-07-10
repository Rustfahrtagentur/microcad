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

    /// Get value by type
    pub fn get_by_type(&self, ty: &Type) -> Option<(&Identifier, &ArgumentValue)> {
        self.map.iter().find(|(_, arg)| arg.value.ty() == *ty)
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
        MultiArgumentMap::find_match(self, &parameters)
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

impl FromIterator<(Identifier, ArgumentValue)> for ArgumentValueList {
    fn from_iter<T: IntoIterator<Item = (Identifier, ArgumentValue)>>(iter: T) -> Self {
        Self {
            map: iter.into_iter().collect(),
            src_ref: SrcRef(None),
        }
    }
}
