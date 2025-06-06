// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Object attributes module
//!
use crate::{syntax::*, value::*};
use microcad_core::ExportSettings;

/// A single object attribute.
#[derive(Clone)]
pub enum MetaDataItem {
    /// Auxiliary object.
    Aux,
    /// A simple color attribute, e.g. `#[color = "#ff00ff", stroke_color = "blue", fill_color = std::colors::white]`.
    Color(Identifier, Color),
    /// An export attribute, e.g. from `#export("filename.svg")` or `node.export("filename.svg")`.
    Export(ExportSettings),
    /// Part id
    Part(Value),
    /// Layer id
    Layer(Value),
}

impl MetaDataItem {
    /// Return id (name) of this object attribute.
    pub fn id(&self) -> Identifier {
        match self {
            MetaDataItem::Aux => Identifier::no_ref("aux"),
            MetaDataItem::Color(identifier, _) => identifier.clone(),
            MetaDataItem::Export(_) => Identifier::no_ref("export"),
            MetaDataItem::Layer(_) => Identifier::no_ref("layer"),
            MetaDataItem::Part(_) => Identifier::no_ref("part"),
        }
    }

    /// Return the value of this object attribute.
    ///
    /// Tag and call attributes like [ObjectAttribute::Aux] will return [Value::None].
    pub fn value(&self) -> Value {
        match self {
            MetaDataItem::Color(_, color) => Value::Color(*color),
            MetaDataItem::Part(value) | MetaDataItem::Layer(value) => value.clone(),
            _ => Value::None,
        }
    }
}

/// Object attribute list.
#[derive(Clone, Default)]
pub struct MetaData(Vec<MetaDataItem>);

impl MetaData {
    /// Merge two [ObjectAttribute] lists.
    ///
    /// # Todo
    /// * Sort this list by id and filter duplicates.
    /// * Decide whether attributes from `self` are updated by `other` or kept.
    pub fn merge(&mut self, other: &mut MetaData) {
        self.0.append(&mut other.0);
    }

    /// Get attribute by id
    pub fn get_by_id(&self, id: &Identifier) -> Option<&MetaDataItem> {
        self.0.iter().find(|attr| &attr.id() == id)
    }
}

impl std::ops::Deref for MetaData {
    type Target = Vec<MetaDataItem>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for MetaData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
