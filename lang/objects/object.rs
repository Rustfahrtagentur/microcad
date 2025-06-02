// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Object.

use crate::{objects::*, syntax::*, value::*};

/// An object with properties
#[derive(Clone, Default)]
pub struct Object {
    /// Name of the object
    pub id: Identifier,

    /// Properties
    pub props: ObjectProperties,

    /// Attributes
    pub attributes: ObjectAttributes,
}

impl Object {
    /// Get object property value.
    pub fn get_property_value(&self, id: &Identifier) -> Option<&Value> {
        self.props.get_value(id)
    }

    /// Assign new object attributes.
    pub fn assign_object_attributes(&mut self, attributes: &mut ObjectAttributes) {
        self.attributes.merge(attributes);
    }

    /// Get value for name-value attribute with `id`.
    pub(crate) fn get_attribute_value(&self, id: &Identifier) -> Option<Value> {
        self.attributes
            .get_by_id(id)
            .map(|attribute| attribute.value())
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}:", self.id)?;
        for (id, value) in self.props.iter() {
            writeln!(f, "\t{id} = {value}")?;
        }

        Ok(())
    }
}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Object {}:", self.id)?;
        for (id, value) in self.props.iter() {
            writeln!(f, "\t{id} = {value}")?;
        }

        Ok(())
    }
}
