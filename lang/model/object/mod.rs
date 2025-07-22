// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Object.

use crate::{model::*, syntax::*, value::*};

mod object_properties;

pub use object_properties::*;

/// An object with properties.
#[derive(Clone, Default)]
pub struct Object {
    /// Properties
    pub props: ObjectProperties,
}

impl Properties for Object {
    fn get_property(&self, id: &Identifier) -> Option<&Value> {
        self.props.get_property(id)
    }

    fn set_property(&mut self, id: Identifier, value: Value) {
        self.props.set_property(id, value)
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
