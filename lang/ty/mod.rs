// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad syntax elements of types

mod matrix_type;
mod quantity_type;
mod tuple_type;
mod r#type;
mod type_list;

pub use matrix_type::*;
pub use quantity_type::*;
pub use tuple_type::*;
pub use r#type::*;
pub use type_list::*;

/// Trait for structs and expressions that have a type
pub trait Ty {
    /// Return type
    fn ty(&self) -> Type;
}
