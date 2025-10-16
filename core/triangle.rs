// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Generic triangle.

/// Triangle
#[derive(Clone, Copy, Debug)]
pub struct Triangle<T>(pub T, pub T, pub T);

/// Implementation for indexed triangle.
impl Triangle<u32> {
    /// A triangle is generated if it contains any repeated index.
    pub fn is_degenerated(&self) -> bool {
        self.0 == self.1 || self.1 == self.2 || self.2 == self.0
    }
}
