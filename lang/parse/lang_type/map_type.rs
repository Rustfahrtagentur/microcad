// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Map type parser entity

use crate::{parse::*, parser::*, r#type::*};

/// Map type (e.g. `[scalar => string]`)
#[derive(Debug, Clone, PartialEq)]
pub struct MapType(MapKeyType, Box<Type>);

impl MapType {
    /// create new map type
    pub fn new(key: MapKeyType, value: Type) -> Self {
        Self(key, Box::new(value))
    }
}

impl Parse for MapType {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut inner = pair.inner();
        let key = inner.next().expect("missing key expression");
        let value = inner.next().expect("missing value expression");

        use crate::Ty;

        Ok(Self::new(
            (TypeAnnotation::parse(key)?.ty()).try_into()?,
            TypeAnnotation::parse(value)?.ty(),
        ))
    }
}

impl std::fmt::Display for MapType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{} => {}]", self.0, self.1)
    }
}
