// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parameter value list evaluation entity

use compact_str::CompactStringExt;

use crate::{eval::*, value::*};

/// List of parameter values
#[derive(Clone, Debug, Default)]
pub struct ParameterValueList(std::collections::HashMap<Identifier, ParameterValue>);

impl ParameterValueList {
    /// Push parameter value
    pub fn insert(
        &mut self,
        id: Identifier,
        parameter: ParameterValue,
    ) -> std::result::Result<(), ValueError> {
        if self.0.contains_key(&id) {
            return Err(ValueError::DuplicateParameter(id.clone()));
        }
        self.0.insert(id, parameter);
        Ok(())
    }

    pub fn get_by_type(&self, ty: Type) -> EvalResult<&ParameterValue> {
        let pv: Vec<_> = self
            .0
            .iter()
            .filter(|(id, v)| id.is_none() && v.type_matches(&ty))
            .collect();
        match pv.len() {
            0 => Err(EvalError::ParameterByTypeNotFound(ty)),
            1 => Ok(pv.first().unwrap().1),
            _ => unreachable!("Type '{ty}' is ambiguous in parameters"),
        }
    }

    /// Check for missing arguments.
    ///
    /// Checks if parameter value list is not empty and wraps the list into an error
    pub fn check_for_missing_arguments(self) -> EvalResult<()> {
        if !self.is_empty() {
            Err(EvalError::MissingArguments(self))
        } else {
            Ok(())
        }
    }
}

impl std::ops::Deref for ParameterValueList {
    type Target = std::collections::HashMap<Identifier, ParameterValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ParameterValueList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for ParameterValueList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            self.0
                .iter()
                .map(|(id, _)| id.to_string())
                .join_compact(", ")
        )
    }
}

impl<I, P> FromIterator<(I, P)> for ParameterValueList
where
    I: Into<Identifier>,
    P: Into<ParameterValue>,
{
    fn from_iter<T: IntoIterator<Item = (I, P)>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|(i, p)| (i.into(), p.into()))
                .collect(),
        )
    }
}
/*h
impl<A: Into<ParameterValue>> FromIterator<(Identifier, A)> for ParameterValueList {
    fn from_iter<T: IntoIterator<Item = (Identifier, A)>>(iter: T) -> Self {
        ParameterValueList(iter.into_iter().map(|(id, pv)| (id, pv.into())).collect())
    }
}
*/
