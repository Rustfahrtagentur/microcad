// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Convert into value

use crate::{eval::*, src_ref::*};

/// Trait to convert something into a value, with an optional origin source reference
pub trait IntoValue {
    /// Convert self into a value with a `SrcRef`
    fn into_value(self, src_ref: SrcRef) -> Value;
}

/// Convert a vector of Vec2 into a value.
impl IntoValue for Vec<microcad_core::Vec2> {
    fn into_value(self, src_ref: SrcRef) -> Value {
        let value_list = ValueList::new(
            self.iter()
                .map(|v| Value::Vec2(Refer::new(*v, src_ref.clone())))
                .collect::<Vec<_>>(),
            src_ref.clone(),
        );

        Value::List(List::new(value_list, crate::r#type::Type::Vec2, src_ref))
    }
}
