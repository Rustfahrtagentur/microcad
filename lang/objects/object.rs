// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Object an object tree

use crate::{value::{SortedValueList, Value}, Id};

/// An object with properties
#[derive(Clone, Default)]
pub struct Object {
    /// Properties
    pub props: SortedValueList,
}

impl Object {
    /// Get object property value
    pub fn get_property_value(&self, id: &Id) -> Option<&Value> {
        self.props.get_value(id)
    }
}

