// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Object an object tree

use std::collections::BTreeMap;

use crate::{value::Value, Id};
use crate::{eval::{EvalContext, EvalResult}, syntax::ParameterList};


/// A list of values sorted by id 
/// 
/// It is required that properties are always sorted by their id. 
#[derive(Clone, Default, Debug)]
pub struct ObjectProperties(BTreeMap<crate::Id, super::Value>);

impl ObjectProperties {
    /// Create initial property list by evaluating parameter list
    pub fn from_parameter_list(parameter_list: &ParameterList, context: &mut EvalContext) -> EvalResult<Self> {
        let mut props = BTreeMap::new();
        for parameter in parameter_list.iter() {
            props.insert(parameter.name.id().clone(), parameter.eval_default_value(context)?);
        }

        Ok(Self(props))
    }

    /// Test if each property has a value
    pub fn is_complete(&self) -> bool {
        self.0.iter().all(|(_, value)| !value.is_invalid())
    }

    /// Get value at id
    pub fn get_value(&self, id: &crate::Id) -> Option<&Value> {
        self.0.get(id)
    }

    /// Get mutable value at id
    pub fn get_value_mut(&mut self, id: &crate::Id) -> Option<&mut Value> {
        self.0.get_mut(id)

    }

    /// Get all ids where `value == Value::None`
    pub fn get_incomplete_ids(&self) -> Vec<crate::Id> {
        self.0.iter().filter_map(|(id, value)| if value.is_invalid() { Some(id.clone()) } else {None}).collect()
    }

    /// If the propery with `id` exists, assign the new value and add as local value to the context 
    pub fn assign_and_add_local_value(&mut self, id: &crate::Id, value: Value, context: &mut EvalContext) {
        if let Some(prop_value) = self.get_value_mut(id) {
            *prop_value = value.clone();
        }
        
        // The result of the assignment becomes a local value, too
        context.add_local_value(id.clone(), value);
    }
}



/// An object with properties
#[derive(Clone, Default)]
pub struct Object {
    /// Properties
    pub props: ObjectProperties,
}

impl Object {
    /// Get object property value
    pub fn get_property_value(&self, id: &Id) -> Option<&Value> {
        self.props.get_value(id)
    }
}

