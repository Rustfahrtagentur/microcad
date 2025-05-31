// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Object properties.

use crate::{syntax::*, value::*};

/// A list of values sorted by id
///
/// It is required that properties are always sorted by their id.
#[derive(Clone, Default, Debug)]
pub struct ObjectProperties(std::collections::HashMap<Identifier, Value>);

impl std::ops::Deref for ObjectProperties {
    type Target = std::collections::HashMap<Identifier, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ObjectProperties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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

    /// Insert property
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

impl std::fmt::Display for ObjectProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ObjectProperties:")?;
        for (id, value) in self.0.iter() {
            writeln!(f, "\t{id:?} = {value:?}")?;
        }

        Ok(())
    }
}
