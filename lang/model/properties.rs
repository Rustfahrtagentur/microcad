// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Object properties.

use crate::{syntax::*, value::*};
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
        writeln!(f, "ObjectProperties:")?;
        for (id, value) in self.0.iter() {
            writeln!(f, "\t{id:?} = {value:?}")?;
        }

        Ok(())
    }
}

/// Access a value of a property by id.
pub trait PropertiesAccess {
    /// Get a value of property, or [`Value::None`] if the property does not exist.
    fn get_property(&self, id: &Identifier) -> Option<&Value>;
    /// Set or create properties with the given ids and values.
    fn add_properties(&mut self, props: Properties);
}
