// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Render cache.

use microcad_core::{Geometry2D, Geometry3D};

/// Render cache item.
#[derive(Debug, Clone)]
pub enum RenderCacheItem {
    /// 2d geometry.
    Geometry2D(Geometry2D),
    /// 3d geometry.
    Geometries3D(Geometry3D),
}
/// Render cache structure.
pub struct RenderCache(pub std::collections::HashMap<crate::Hash, RenderCacheItem>);
