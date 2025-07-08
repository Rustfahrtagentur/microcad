// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Node attributes collection.

use derive_more::{Deref, DerefMut};

use crate::{model_tree::*, syntax::Identifier};

/// Node attributes, from an evaluated attribute list.
#[derive(Clone, Debug, Default, Deref, DerefMut)]
pub struct Attributes(Vec<Attribute>);

impl Attributes {
    /// Create new attributes from attribute.
    pub fn new(attributes: Vec<Attribute>) -> Self {
        Self(attributes)
    }
}

impl GetAttribute for Attributes {
    fn get_attribute(&self, id: &Identifier) -> Option<Attribute> {
        self.0
            .iter()
            .find(|attribute| *id == attribute.id())
            .cloned()
    }
}
