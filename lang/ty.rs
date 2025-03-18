// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Type trait

use crate::r#type::Type;

/// Trait for structs and expressions that have a type
pub trait Ty {
    /// Return type
    fn ty(&self) -> Type;
}
