// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Object attributes module
//!
use crate::syntax::*;
use microcad_core::ExportSettings;

/// A single object attribute.
#[derive(Clone)]
pub enum ObjectAttribute {
    /// A simple color attribute, e.g. `#[color = "#ff00ff", stroke_color = "blue", fill_color = std::colors::white]`.
    Color(Identifier, Color),
    /// An export attribute, e.g. from `#export("filename.svg")` or `node.export("filename.svg")`.
    Export(ExportSettings),
}

impl ObjectAttribute {
    /// Return id of this object attribute.
    pub fn id(&self) -> Identifier {
        match self {
            ObjectAttribute::Color(identifier, _) => identifier.clone(),
            ObjectAttribute::Export(_) => Identifier::no_ref("export"),
        }
    }
}

/// Object attribute list.
#[derive(Clone, Default)]
pub struct ObjectAttributes(Vec<ObjectAttribute>);

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
