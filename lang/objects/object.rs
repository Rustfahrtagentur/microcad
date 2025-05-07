// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Object an object tree

use crate::{
    resolve::{SymbolDefinition, SymbolMap},
    syntax::*,
    value::*,
};
use std::collections::BTreeMap;

/// A list of values sorted by id
///
/// It is required that properties are always sorted by their id.
#[derive(Clone, Default, Debug)]
pub struct ObjectProperties(BTreeMap<Identifier, Value>);

impl ObjectProperties {
    /// Test if each property has a value
    pub fn all_initialized(&self) -> bool {
        self.0.iter().all(|(_, value)| !value.is_invalid())
    }

    /// Get value at id
    pub fn get_value(&self, id: &Identifier) -> Option<&Value> {
        self.0.get(id)
    }

    /// Get mutable value at id
    pub fn get_value_mut(&mut self, id: &Identifier) -> Option<&mut Value> {
        self.0.get_mut(id)
    }

    pub fn insert(&mut self, id: Identifier, value: Value) {
        self.0.insert(id, value);
    }

    /// Get all ids where `value == Value::None`
    pub fn get_ids_of_uninitialized(&self) -> Vec<Identifier> {
        self.0
            .iter()
            .filter_map(|(id, value)| {
                if value.is_invalid() {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

impl From<&SymbolMap> for ObjectProperties {
    fn from(symbol_map: &SymbolMap) -> Self {
        let mut props = ObjectProperties::default();
        for (id, symbol) in symbol_map.iter() {
            match &symbol.borrow().def {
                SymbolDefinition::Property(_, value) => {
                    props.0.insert(id.clone(), value.clone());
                }
                _ => {}
            }
        }
        props
    }
}

/// An object with properties
#[derive(Clone, Default, Debug)]
pub struct Object {
    /// Name of the object
    pub id: Identifier,

    /// Properties
    pub props: ObjectProperties,
}

impl Object {
    /// Get object property value
    pub fn get_property_value(&self, id: &Identifier) -> Option<&Value> {
        self.props.get_value(id)
    }
}
