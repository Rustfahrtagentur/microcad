// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Convert into value

use crate::value::*;

/// Trait to convert something into a value, with an optional origin source reference
pub trait IntoValue {
    /// Convert self into a value with a `SrcRef`
    fn into_value(self) -> Value;
}
