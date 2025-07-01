// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Object.

use crate::{GetPropertyValue, syntax::*, value::*};

mod object_properties;

pub use object_properties::*;

/// An object with properties.
#[derive(Clone, Default)]
pub struct Object {
    /// Properties
    pub props: ObjectProperties,
}

impl GetPropertyValue for Object {
    fn get_property_value(&self, id: &Identifier) -> Value {
        self.props.get_property_value(id)
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (id, value) in self.props.iter() {
            writeln!(f, "\t{id} = {value}")?;
        }

        Ok(())
    }
}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Object:")?;
        for (id, value) in self.props.iter() {
            writeln!(f, "\t{id} = {value}")?;
        }

        Ok(())
    }
}
