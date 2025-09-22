// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad core geometry traits

use crate::{Alignment, Direction, RenderResolution};

/// Trait to align something to center
pub trait Align<T = Self> {
    /// Align geometry.
    fn align(&self, direction: Direction, alignment: Alignment, resolution: &RenderResolution)
        -> T;
}
