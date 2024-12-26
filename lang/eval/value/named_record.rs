// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Named record evaluation entity

use crate::{eval::*, parse::*, r#type::*, src_ref::*};

/// Short cut to create a NamedRecord
#[cfg(test)]
#[macro_export]
macro_rules! named_tuple {
    ($($name:ident: $ty:ident = $value:expr),*) => {
        NamedRecord::from_vec(vec![$((stringify!($name).into(), Value::$ty($value)),)*])
    };
}

/// Record with named values
#[derive(Clone, Debug, PartialEq)]
pub struct NamedRecord(Refer<std::collections::BTreeMap<Identifier, Value>>);

impl NamedRecord {
    /// Create new named record instance
    pub fn new(map: std::collections::BTreeMap<Identifier, Value>, src_ref: SrcRef) -> Self {
        Self(Refer::new(map, src_ref))
    }
}

impl SrcReferrer for NamedRecord {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

impl std::ops::Deref for NamedRecord {
    type Target = std::collections::BTreeMap<Identifier, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for NamedRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for NamedRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({items})",
            items = self
                .0
                .iter()
                .map(|(k, v)| format!("{k} => {v}"))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl Ty for NamedRecord {
    fn ty(&self) -> Type {
        Type::NamedRecord(NamedRecordType(
            self.0
                .iter()
                .map(|(name, v)| (name.clone(), v.ty().clone()))
                .collect(),
        ))
    }
}
