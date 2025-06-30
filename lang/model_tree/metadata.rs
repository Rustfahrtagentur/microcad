// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Object attributes module

use crate::{syntax::*, value::*};

/// Node metadata, from an evaluated attribute list.
#[derive(Clone, Debug, Default)]
pub struct Attributes(pub(crate) std::collections::BTreeMap<Identifier, Value>);

impl Attributes {
    /// Set export data for node
    ///
    /// Add an export attribute with µcad syntax:
    ///
    /// ```ucad
    /// #[export("filename.svg")]
    /// std::geo2d::circle(r = 42mm).
    /// ```
    ///
    /// Add an attribute using the model node API:
    ///
    /// ```
    /// let mut node = source_file.eval("source.µcad");
    /// node.attributes_mut().put("export", tuple!("filename.svg"))
    /// ```
    pub fn set_export(&mut self, settings: Tuple) -> &mut Self {
        self.put(&Identifier::no_ref("export"), settings)
    }

    /// Put a value into the tuple.
    ///
    /// TODO:
    /// [ ] Error handling when id already exists.
    pub fn put<T: Into<Value>>(&mut self, id: &Identifier, value: T) -> &mut Self {
        self.0.insert(id.clone(), value.into());
        self
    }

    /// Get attribute value
    pub fn get(&self, id: &Identifier) -> Option<&Value> {
        self.0.get(id)
    }

    /// Get attribute value as a tuple.
    pub fn get_as_tuple(&self, id: &Identifier) -> Option<&Tuple> {
        match self.get(id) {
            Some(Value::Tuple(tuple)) => Some(tuple),
            Some(_) | None => None,
        }
    }
}

impl std::ops::Deref for Attributes {
    type Target = std::collections::BTreeMap<Identifier, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Attributes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
