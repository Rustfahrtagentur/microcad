// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad core geometry traits

/// Trait to align something to center
///
/// TODO: This trait might be extended so that.
pub trait Align<T = Self> {
    /// Align geometry.
    fn align(&self) -> T;
}
