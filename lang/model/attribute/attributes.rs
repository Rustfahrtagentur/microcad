// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model attributes collection.

use derive_more::{Deref, DerefMut};

use crate::{model::*, syntax::Identifier};

/// Model attributes, from an evaluated attribute list.
///
/// The model attributes can be produced from:
/// * outer attributes: `#[export = "test.svg"]`
/// * inner attributes: `#![export = "test.svg"]`
///
#[derive(Clone, Debug, Default, Deref, DerefMut, serde::Serialize, serde::Deserialize)]
pub struct Attributes(pub Vec<Attribute>);

impl AttributesAccess for Attributes {
    fn get_attributes_by_id(&self, id: &Identifier) -> Vec<Attribute> {
        self.iter()
            .filter(|attribute| attribute.id() == *id)
            .cloned()
            .collect()
    }
}

impl FromIterator<Attribute> for Attributes {
    fn from_iter<T: IntoIterator<Item = Attribute>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl TreeDisplay for Attributes {
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result {
        self.iter()
            .try_for_each(|attribute| writeln!(f, "{:depth$}{attribute}", ""))
    }
}
