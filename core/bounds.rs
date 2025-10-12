// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

#[derive(Debug, Clone)]
pub struct Bounds<T> {
    min: T,
    max: T,
}

impl<T> Bounds<T> {
    fn new(min: T, max: T) -> Self {
        Self { min, max }
    }

    /// Minimum corner.
    pub fn min(&self) -> &T {
        &self.min
    }

    /// Maximum corner.
    pub fn max(&self) -> &T {
        &self.max
    }
}
