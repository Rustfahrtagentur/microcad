// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Render cache.

use microcad_core::{Geometry2D, Geometry3D};

/// Render hash type.
pub type RenderHash = u64;

/// An item in the [`RenderCache`].
pub enum RenderCacheItem {
    /// 2D geometry.
    Geometry2D(Geometry2D),
    /// 3D geometry.
    Geometry3D(Geometry3D),
}

/// The [`RenderCache`] owns all geometry created during the render process.
pub struct RenderCache(std::collections::HashMap<RenderHash, RenderCacheItem>);

impl RenderCache {
    /// Create a new empty cache.
    pub fn new() -> Self {
        Self(Default::default())
    }

    /// Empty cache.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Get 2D geometry from the cache.
    pub fn get_2d(&self, hash: RenderHash) -> Option<&Geometry2D> {
        match self.0.get(&hash) {
            Some(RenderCacheItem::Geometry2D(g)) => Some(g),
            _ => None,
        }
    }

    /// Get 3D geometry from the cache.
    pub fn get_3d(&self, hash: RenderHash) -> Option<&Geometry3D> {
        match self.0.get(&hash) {
            Some(RenderCacheItem::Geometry3D(g)) => Some(g),
            _ => None,
        }
    }

    /// Insert 2D geometry into the cache.
    pub fn insert_2d(&mut self, hash: RenderHash, geo2d: Geometry2D) {
        self.0.insert(hash, RenderCacheItem::Geometry2D(geo2d));
    }

    /// Insert 3D geometry into the cache.
    pub fn insert_3d(&mut self, hash: RenderHash, geo3d: Geometry3D) {
        self.0.insert(hash, RenderCacheItem::Geometry3D(geo3d));
    }
}

impl Default for RenderCache {
    fn default() -> Self {
        Self::new()
    }
}
