// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Sorted value list

use crate::{eval::{EvalContext, EvalResult}, syntax::ParameterList};

use super::Value;

/// A list of values sorted by id 
/// 
/// It is required that properties are always sorted by their id. 
#[derive(Clone, Default, Debug)]
pub struct SortedValueList(Vec<(crate::Id, super::Value)>);

impl SortedValueList {
    /// Create initial property list by evaluating parameter list
    pub fn from_parameter_list(parameter_list: &ParameterList, context: &mut EvalContext) -> EvalResult<Self> {
        let mut props = Vec::new();
        for parameter in parameter_list.iter() {
            props.push((parameter.name.id().clone(), parameter.eval_default_value(context)?));
        }

        props.sort_by(|(lhs, _), (rhs, _)| lhs.cmp(rhs));

        Ok(Self(props))
    }

    /// Test if each property has a value
    pub fn is_complete(&self) -> bool {
        self.0.iter().all(|(_, value)| !value.is_invalid())
    }

    /// Get index of item
    pub fn get_index(&self, id: &crate::Id) -> Result<usize, usize> {
        self.0.binary_search_by(|(prop_id, _)| prop_id.cmp(id))
    }

    /// Get value at id
    pub fn get_value(&self, id: &crate::Id) -> Option<&Value> {
        match self.get_index(id) {
            Ok(index) => self.0.get(index).map(|p| &p.1),
            Err(_) => None,
        }
    }

    /// Get mutable value at id
    pub fn get_value_mut(&mut self, id: &crate::Id) -> Option<&mut Value> {
        match self.get_index(id) {
            Ok(index) => self.0.get_mut(index).map(|p| &mut p.1),
            Err(_) => None,
        }
    }

    /// Get all ids where `value == Value::None`
    pub fn get_incomplete_ids(&self) -> Vec<crate::Id> {
        self.0.iter().filter_map(|(id, value)| if value.is_invalid() { Some(id.clone()) } else {None}).collect()
    }

    /// Assign the new value to the respective id if present and add as local value to the context 
    pub fn assign_and_add_local_value(&mut self, id: &crate::Id, value: Value, context: &mut EvalContext) {
        if let Some(prop_value) = self.get_value_mut(id) {
            *prop_value = value.clone();
        }
        
        // The result of the assignment becomes a local value, too
        context.add_local_value(id.clone(), value);
    }
}

