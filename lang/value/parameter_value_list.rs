// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parameter value list evaluation entity

use compact_str::CompactStringExt;

use crate::{src_ref::*, value::*};

/// List of parameter values
#[derive(Clone, Debug, Default)]
pub struct ParameterValueList {
    /// List of parameter values
    parameters: Vec<ParameterValue>,
    /// access map by id
    pub by_id: std::collections::BTreeMap<Identifier, usize>,
    /// Source code reference
    src_ref: SrcRef,
}

impl ParameterValueList {
    /// Create new ParameterValueList
    #[cfg(test)]
    pub fn new(parameters: Vec<ParameterValue>) -> Self {
        let mut by_id = std::collections::BTreeMap::new();
        for (i, parameter) in parameters.iter().enumerate() {
            by_id.insert(parameter.id.clone(), i);
        }

        Self {
            by_id,
            src_ref: SrcRef::from_vec(&parameters),
            parameters,
        }
    }

    /// Push parameter value
    pub fn push(&mut self, parameter: ParameterValue) -> std::result::Result<(), ValueError> {
        if self.by_id.contains_key(&parameter.id) {
            return Err(ValueError::DuplicateParameter(parameter.id.clone()));
        }

        self.by_id
            .insert(parameter.id.clone(), self.parameters.len());
        self.parameters.push(parameter);
        Ok(())
    }

    /// get ParameterValue by id
    pub fn get_by_id(&self, id: &Identifier) -> Option<&ParameterValue> {
        self.by_id.get(id).map(|i| &self.parameters[*i])
    }

    /// get ParameterValue by index
    pub fn get_by_index(&self, index: usize) -> Option<&ParameterValue> {
        self.parameters.get(index)
    }

    /// remove parameter value by id
    pub fn remove(&mut self, id: &Identifier) {
        if let Some(new_index) = self.by_id.remove(id) {
            self.parameters.remove(new_index);
            self.by_id
                .values_mut()
                .skip(new_index)
                .for_each(|index| *index -= 1);
        }
    }

    /// Return `true` if empty
    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }

    /// Check for missing arguments.
    ///
    /// Checks if parameter value list is not empty and wraps the list into an error
    pub fn check_for_missing_arguments(self) -> Result<(), ValueError> {
        if !self.is_empty() {
            Err(ValueError::MissingArguments(self))
        } else {
            Ok(())
        }
    }
}

impl SrcReferrer for ParameterValueList {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::ops::Deref for ParameterValueList {
    type Target = Vec<ParameterValue>;

    fn deref(&self) -> &Self::Target {
        &self.parameters
    }
}

impl std::fmt::Display for ParameterValueList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            self.parameters
                .iter()
                .map(|p| p.id.to_string())
                .join_compact(", ")
        )
    }
}
