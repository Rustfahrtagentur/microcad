// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Call argument value evaluation entity

use crate::{Id, ord_map::*, src_ref::*, value::*};

/// Call argument value
#[derive(Clone, Debug)]
pub struct CallArgumentValue {
    /// Argument name
    pub name: Option<Id>,
    /// Argument value
    pub value: Value,
    /// Source code reference
    src_ref: SrcRef,
}

impl OrdMapValue<Id> for CallArgumentValue {
    fn key(&self) -> Option<Id> {
        self.name.clone()
    }
}

impl SrcReferrer for CallArgumentValue {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl CallArgumentValue {
    /// Create new call argument value
    pub fn new(name: Option<Id>, value: Value, src_ref: SrcRef) -> Self {
        Self {
            name,
            value,
            src_ref,
        }
    }
}

/// Shortcut to create a argument value
#[macro_export]
#[cfg(test)]
macro_rules! call_argument_value {
    ($name:ident: $ty:ident = $value:expr) => {
        CallArgumentValue::new(
            Some(stringify!($name).into()),
            Value::$ty($crate::src_ref::Refer::none($value)),
            $crate::src_ref::SrcRef(None),
        )
    };
    ($ty:ident = $value:expr) => {
        CallArgumentValue::new(
            None,
            Value::$ty($crate::src_ref::Refer::none($value)),
            $crate::src_ref::SrcRef(None),
        )
    };
    () => {};
}
