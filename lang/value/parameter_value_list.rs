// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parameter value list evaluation entity

use compact_str::CompactStringExt;

use crate::{rc::*, value::*};

/// List of parameter values
#[derive(Clone, Debug, Default)]
pub struct ParameterValueList {
    /// List of parameter values
    parameters: Vec<Rc<ParameterValue>>,
    /// access map by id
    by_id: std::collections::BTreeMap<Identifier, usize>,
    /// access map by type
    by_type: std::collections::BTreeMap<Type, Vec<usize>>,
}

impl ParameterValueList {
    /// Create new ParameterValueList
    #[cfg(test)]
    pub fn new(parameters: Vec<ParameterValue>) -> Self {
        let mut by_id = std::collections::BTreeMap::new();
        let mut by_type: std::collections::BTreeMap<Type, Vec<usize>> =
            std::collections::BTreeMap::new();
        for (i, parameter) in parameters.iter().enumerate() {
            by_id.insert(parameter.id.clone(), i);
            if let Some((_, v)) = by_type.iter_mut().find(|(ty, _)| **ty == parameter.ty()) {
                v.push(i);
            } else {
                by_type.insert(parameter.ty(), vec![i]);
            }
        }

        Self {
            by_id,
            by_type,
            parameters: parameters.into_iter().map(Rc::new).collect(),
        }
    }

    /// Push parameter value
    pub fn push(&mut self, parameter: ParameterValue) -> std::result::Result<(), ValueError> {
        if self.by_id.contains_key(&parameter.id) {
            return Err(ValueError::DuplicateParameter(parameter.id.clone()));
        }

        let pos = self.parameters.len();
        self.by_id.insert(parameter.id.clone(), pos);
        if let Some((_, v)) = self
            .by_type
            .iter_mut()
            .find(|(ty, _)| **ty == parameter.ty())
        {
            v.push(pos);
        } else {
            log::trace!("pos = {pos}");
            self.by_type.insert(parameter.ty(), vec![pos]);
        }
        self.parameters.push(Rc::new(parameter));
        Ok(())
    }

    /// get ParameterValue by id
    pub fn get_by_id(&self, id: &Identifier) -> Option<Rc<ParameterValue>> {
        self.by_id.get(id).map(|i| self.parameters[*i].clone())
    }

    /// get ParameterValue by id
    pub fn get_by_type(&self, ty: &Type) -> Vec<Rc<ParameterValue>> {
        self.by_type
            .iter()
            .find_map(|(t, indizes)| {
                if ty.can_convert_into(t) {
                    Some(
                        indizes
                            .iter()
                            .map(|i| self.parameters[*i].clone())
                            .collect::<Vec<_>>(),
                    )
                } else {
                    None
                }
            })
            .unwrap_or(vec![])
    }

    /// get ParameterValue by index
    pub fn get_by_index(&self, index: usize) -> Option<&Rc<ParameterValue>> {
        self.parameters.get(index)
    }

    /// remove parameter value by id
    pub fn remove(&mut self, id: &Identifier) {
        if let Some(index) = self.by_id.remove(id) {
            self.parameters.remove(index);
            self.by_id
                .values_mut()
                .skip(index)
                .for_each(|index| *index -= 1);
            self.by_type.values_mut().for_each(|v| {
                if let Some(idx) = v.iter().position(|idx| *idx == index) {
                    v.remove(idx);
                }
                v.iter_mut()
                    .filter(|idx| **idx > index)
                    .for_each(|idx| *idx -= 1);
            });
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

impl std::ops::Deref for ParameterValueList {
    type Target = Vec<Rc<ParameterValue>>;

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

impl From<Vec<Rc<ParameterValue>>> for ParameterValueList {
    fn from(parameters: Vec<Rc<ParameterValue>>) -> Self {
        let mut by_id = std::collections::BTreeMap::new();
        let mut by_type: std::collections::BTreeMap<Type, Vec<usize>> =
            std::collections::BTreeMap::new();
        for (i, parameter) in parameters.iter().enumerate() {
            by_id.insert(parameter.id.clone(), i);
            if let Some((_, v)) = by_type.iter_mut().find(|(ty, _)| **ty == parameter.ty()) {
                v.push(i);
            } else {
                by_type.insert(parameter.ty(), vec![i]);
            }
        }

        Self {
            by_id,
            by_type,
            parameters: parameters.into_iter().collect(),
        }
    }
}
