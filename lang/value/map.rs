// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Map type evaluation entity

use crate::{parse::*, r#type::*, value::*};

/// Map evaluation entity
#[derive(Debug, Clone, PartialEq)]
pub struct Map {
    /// Map
    pub map: std::collections::HashMap<MapKeyValue, Value>,
    /// Key type of the map
    pub key_type: MapKeyType,
    /// Type of the map's values
    pub ty: Type,
    /// Source code reference
    src_ref: SrcRef,
}

impl SrcReferrer for Map {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl From<Map> for std::collections::HashMap<MapKeyValue, Value> {
    fn from(val: Map) -> Self {
        val.map
    }
}

impl crate::ty::Ty for Map {
    fn ty(&self) -> Type {
        self.ty.clone()
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[{items}]",
            items = self
                .map
                .iter()
                .map(|(k, v)| format!("{k} => {v}"))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
