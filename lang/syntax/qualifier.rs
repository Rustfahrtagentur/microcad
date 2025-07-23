// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Qualifier of an assignment

/// Qualifier of an assignment
///
/// This is used to determine if an entity is public or private.
/// By default, entities are private.
#[derive(Clone, Debug, Default)]
pub enum Qualifier {
    /// local variable
    #[default]
    LocalVar,
    /// public constant
    Constant,
    /// workbench property
    Property,
}
