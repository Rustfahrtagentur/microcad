// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Convert into value

use crate::value::*;

/// Trait to convert something into a value, with an optional origin source reference
pub trait IntoValue {
    /// Convert self into a value with a `SrcRef`
    fn into_value(self) -> Value;
}

/// Convert a vector of Vec2 into a value.
impl IntoValue for Vec<microcad_core::Vec2> {
    fn into_value(self) -> Value {
        let value_list = ValueList::new(self.iter().map(|v| Value::Vec2(*v)).collect::<Vec<_>>());

        Value::List(List::new(value_list, crate::ty::Type::Vec2))
    }
}
