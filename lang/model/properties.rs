// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Object properties.

use crate::{syntax::*, tree_display::*, value::*};
use derive_more::{Deref, DerefMut};
use std::collections::BTreeMap;

/// A list of object properties.
///
/// It is required that properties are always sorted by their id.
#[derive(Clone, Default, Debug, Deref, DerefMut)]
pub struct Properties(BTreeMap<Identifier, Value>);

impl FromIterator<(Identifier, Value)> for Properties {
    fn from_iter<T: IntoIterator<Item = (Identifier, Value)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Properties {
    /// Test if each property has a value.
    pub fn is_valid(&self) -> bool {
        self.0.iter().all(|(_, value)| !value.is_invalid())
    }
}

impl std::fmt::Display for Properties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Properties:")?;
        for (id, value) in self.0.iter() {
            writeln!(f, "\t{id:?} : {value:?}")?;
        }

        Ok(())
    }
}

impl TreeDisplay for Properties {
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result {
        self.iter()
            .try_for_each(|(id, value)| writeln!(f, "{:depth$}- {id:?} = {value:?}", ""))
    }
}

/// Access a value of a property by id.
pub trait PropertiesAccess {
    /// Get a value of property, or [`Value::None`] if the property does not exist.
    fn get_property(&self, id: &Identifier) -> Option<&Value>;
    /// Set value of an existing property or add a new property
    fn set_property(&mut self, id: Identifier, value: Value) -> Option<Value>;
    /// Get all properties
    fn get_properties(&self) -> Option<&Properties>;
    /// Set or create properties with the given ids and values.
    fn add_properties(&mut self, props: Properties);
}
