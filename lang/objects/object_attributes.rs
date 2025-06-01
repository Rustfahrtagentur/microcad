// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Object attributes module
//!
use crate::{syntax::*, value::*};
use microcad_core::ExportSettings;

/// A single object attribute.
#[derive(Clone)]
pub enum ObjectAttribute {
    /// Auxiliary object.
    Aux,
    /// A simple color attribute, e.g. `#[color = "#ff00ff", stroke_color = "blue", fill_color = std::colors::white]`.
    Color(Identifier, Color),
    /// An export attribute, e.g. from `#export("filename.svg")` or `node.export("filename.svg")`.
    Export(ExportSettings),
    /// Part id
    PartId(Value),
    /// Layer id
    Layer(Value),
}

impl ObjectAttribute {
    /// Return id (name) of this object attribute.
    pub fn id(&self) -> Identifier {
        match self {
            ObjectAttribute::Aux => Identifier::no_ref("aux"),
            ObjectAttribute::Color(identifier, _) => identifier.clone(),
            ObjectAttribute::Export(_) => Identifier::no_ref("export"),
            ObjectAttribute::Layer(_) => Identifier::no_ref("layer"),
            ObjectAttribute::PartId(_) => Identifier::no_ref("part_id"),
        }
    }

    /// Return the value of this object attribute.
    ///
    /// Tag and call attributes like [ObjectAttribute::Aux] will return [Value::None].
    pub fn value(&self) -> Value {
        match self {
            ObjectAttribute::Color(_, color) => Value::Color(*color),
            ObjectAttribute::PartId(value) | ObjectAttribute::Layer(value) => value.clone(),
            _ => Value::None,
        }
    }
}

/// Object attribute list.
#[derive(Clone, Default)]
pub struct ObjectAttributes(Vec<ObjectAttribute>);

impl ObjectAttributes {
    /// Merge two [ObjectAttribute] lists.
    ///
    /// # Todo
    /// * Sort this list by id and filter duplicates.
    /// * Decide whether attributes from `self` are updated by `other` or kept.
    pub fn merge(&mut self, other: &mut ObjectAttributes) {
        self.0.append(&mut other.0);
    }

    /// Get attribute by id
    pub fn get_by_id(&self, id: &Identifier) -> Option<&ObjectAttribute> {
        self.0.iter().find(|attr| &attr.id() == id)
    }
}

impl std::ops::Deref for ObjectAttributes {
    type Target = Vec<ObjectAttribute>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ObjectAttributes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
