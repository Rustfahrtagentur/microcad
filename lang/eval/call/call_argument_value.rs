// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Call argument value evaluation entity

use crate::{ord_map::*, src_ref::*, syntax::*, value::*};

/// Call argument value.
#[derive(Clone, Debug)]
pub struct CallArgumentValue {
    /// *id* of the argument.
    pub id: Option<Identifier>,
    /// *value* of the argument.
    pub value: Value,
    /// Source code reference.
    src_ref: SrcRef,
}

impl OrdMapValue<Identifier> for CallArgumentValue {
    fn key(&self) -> Option<Identifier> {
        self.id.clone()
    }
}

impl SrcReferrer for CallArgumentValue {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for CallArgumentValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.id {
            Some(ref id) => write!(f, "{} = {}", id, self.value),
            None => write!(f, "{}", self.value),
        }
    }
}

impl CallArgumentValue {
    /// Create new call argument value
    pub fn new(id: Option<Identifier>, value: Value, src_ref: SrcRef) -> Self {
        Self { id, value, src_ref }
    }
}

impl From<CallArgumentValue> for Value {
    fn from(value: CallArgumentValue) -> Self {
        value.value.clone()
    }
}

impl From<Vec<CallArgumentValue>> for Value {
    fn from(values: Vec<CallArgumentValue>) -> Self {
        match values.len() {
            0 => Value::None,
            1 => values.first().expect("Node").clone().into(),
            _ => values.clone().into(),
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
            $crate::value::Value::$ty($value),
            $crate::src_ref::SrcRef(None),
        )
    };
    ($ty:ident = $value:expr) => {
        CallArgumentValue::new(
            None,
            $crate::value::Value::$ty($value),
            $crate::src_ref::SrcRef(None),
        )
    };
    () => {};
}
