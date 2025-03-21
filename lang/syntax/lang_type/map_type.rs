// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Map type syntax element

use crate::{syntax::*, r#type::*};

/// Map type (e.g. `[scalar => string]`)
#[derive(Debug, Clone, PartialEq)]
pub struct MapType(MapKeyType, Box<Type>);

impl MapType {
    /// create new map type
    pub fn new(key: MapKeyType, value: Type) -> Self {
        Self(key, Box::new(value))
    }
}

impl std::fmt::Display for MapType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{} => {}]", self.0, self.1)
    }
}
