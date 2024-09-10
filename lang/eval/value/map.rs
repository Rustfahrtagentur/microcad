// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use value::{SrcRef, SrcReferrer};

use crate::{eval::*, map_key_type::*, r#type::*};

#[derive(Debug, Clone, PartialEq)]
pub struct Map {
    pub map: std::collections::HashMap<MapKeyValue, Value>,
    pub key_type: MapKeyType,
    pub ty: Type,
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

impl Ty for Map {
    fn ty(&self) -> Type {
        self.ty.clone()
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

