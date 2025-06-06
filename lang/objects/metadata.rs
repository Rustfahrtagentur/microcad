// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Object attributes module
//!
use crate::{syntax::*, value::*};
use microcad_core::ExportSettings;

/// A single [`MetaData`] item
#[derive(Clone)]
pub enum MetadataItem {
    /// The node is auxiliary, it will not be exported.
    Aux,
    /// A simple color attribute, e.g. `#[color = "#ff00ff", stroke_color = "blue", fill_color = std::colors::white]`.
    Color(Identifier, Color),
    /// An export attribute, e.g. from `#export("filename.svg")` or `node.export("filename.svg")`.
    Export(ExportSettings),
    /// Part id
    ItemId(Value),
    /// Layer id
    Layer(Value),
}

impl MetadataItem {
    /// Return id (name) of this object attribute.
    pub fn id(&self) -> Identifier {
        match self {
            MetadataItem::Aux => Identifier::no_ref("aux"),
            MetadataItem::Color(identifier, _) => identifier.clone(),
            MetadataItem::Export(_) => Identifier::no_ref("export"),
            MetadataItem::Layer(_) => Identifier::no_ref("layer"),
            MetadataItem::ItemId(_) => Identifier::no_ref("item_id"),
        }
    }

    /// Return the value of this object attribute.
    ///
    /// Tag and call attributes like [ObjectAttribute::Aux] will return [Value::None].
    pub fn value(&self) -> Value {
        match self {
            MetadataItem::Color(_, color) => Value::Color(*color),
            MetadataItem::ItemId(value) | MetadataItem::Layer(value) => value.clone(),
            _ => Value::None,
        }
    }
}

/// Object attribute list.
#[derive(Clone, Default)]
pub struct Metadata(Vec<MetadataItem>);

impl Metadata {
    /// Merge two [ObjectAttribute] lists.
    ///
    /// # Todo
    /// * Sort this list by id and filter duplicates.
    /// * Decide whether attributes from `self` are updated by `other` or kept.
    pub fn merge(&mut self, other: &mut Metadata) {
        self.0.append(&mut other.0);
    }

    /// Get attribute by id
    pub fn get_by_id(&self, id: &Identifier) -> Option<&MetadataItem> {
        self.0.iter().find(|attr| &attr.id() == id)
    }
}

impl std::ops::Deref for Metadata {
    type Target = Vec<MetadataItem>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Metadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
