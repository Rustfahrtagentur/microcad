// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad core geometry traits

use crate::RenderResolution;

pub trait BooleanOp<T = Self> {}

pub trait Center<T = Self> {
    fn center(&self, resolution: &RenderResolution) -> T;
}

pub trait Hull<T = Self> {
    fn hull(&self, resolution: &RenderResolution) -> T;
}
