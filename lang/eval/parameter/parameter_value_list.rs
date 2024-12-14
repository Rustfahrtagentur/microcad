// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parameter value list evaluation entity

use crate::{eval::*, src_ref::*};

/// List of parameter values
#[derive(Clone, Debug, Default)]
pub struct ParameterValueList {
    /// List of parameter values
    parameters: Vec<ParameterValue>,
    /// access map by name
    pub by_name: std::collections::BTreeMap<Id, usize>,
    /// Source code reference
    src_ref: SrcRef,
}

impl ParameterValueList {
    /// Create new ParameterValueList
    #[cfg(test)]
    pub fn new(parameters: Vec<ParameterValue>) -> Self {
        let mut by_name = std::collections::BTreeMap::new();
        for (i, parameter) in parameters.iter().enumerate() {
            by_name.insert(parameter.name.clone(), i);
        }

        Self {
            by_name,
            src_ref: SrcRef::from_vec(&parameters),
            parameters,
        }
    }

    /// Push parameter value
    pub fn push(&mut self, parameter: ParameterValue) -> std::result::Result<(), EvalError> {
        if self.by_name.contains_key(&parameter.name) {
            return Err(EvalError::DuplicateParameter(parameter.name.clone()));
        }

        self.by_name
            .insert(parameter.name.clone(), self.parameters.len());
        self.parameters.push(parameter);
        Ok(())
    }

    /// get ParameterValue by name
    pub fn get_by_name(&self, name: &Id) -> Option<&ParameterValue> {
        self.by_name.get(name).map(|i| &self.parameters[*i])
    }

    /// get ParameterValue by index
    pub fn get_by_index(&self, index: usize) -> Option<&ParameterValue> {
        self.parameters.get(index)
    }

    /// remove parameter value by name
    pub fn remove(&mut self, name: &Id) {
        if let Some(new_index) = self.by_name.remove(name) {
            self.parameters.remove(new_index);
            self.by_name
                .values_mut()
                .skip(new_index)
                .for_each(|index| *index -= 1);
        }
    }

    /// Return `true` if empty
    pub fn is_empty(&self) -> bool {
        self.by_name.is_empty()
    }

    /// Check for missing arguments.
    ///
    /// Checks if parameter value list is not empty and wraps the list into an error
    pub fn check_for_missing_arguments(self) -> Result<()> {
        if !self.is_empty() {
            Err(EvalError::MissingArguments(self))
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
