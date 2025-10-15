// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Render cache.

use std::rc::Rc;

use microcad_core::{Geometry2D, Geometry3D};

use crate::render::{Geometry2DOutput, Geometry3DOutput, HashId};

/// An item in the [`RenderCache`].
pub enum RenderCacheItem {
    /// 2D geometry. Note: The Rc can be removed eventually, once the implementation of RenderHash is finished.
    Geometry2D(Geometry2DOutput),
    /// 3D geometry. Note: The Rc can be removed eventually, once the implementation of RenderHash is finished.
    Geometry3D(Geometry3DOutput),
}

/// The [`RenderCache`] owns all geometry created during the render process.
pub struct RenderCache(std::collections::HashMap<HashId, RenderCacheItem>);

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
    pub fn get_2d(&self, hash: &HashId) -> Option<&Geometry2DOutput> {
        match self.0.get(hash) {
            Some(RenderCacheItem::Geometry2D(g)) => Some(g),
            _ => None,
        }
    }

    /// Get 3D geometry from the cache.
    pub fn get_3d(&self, hash: &HashId) -> Option<&Geometry3DOutput> {
        match self.0.get(hash) {
            Some(RenderCacheItem::Geometry3D(g)) => Some(g),
            _ => None,
        }
    }

    /// Insert 2D geometry into the cache and return inserted geometry.
    pub fn insert_2d(&mut self, hash: HashId, geo2d: Geometry2DOutput) -> Geometry2DOutput {
        self.0
            .insert(hash, RenderCacheItem::Geometry2D(geo2d.clone()));
        geo2d
    }

    /// Insert 3D geometry into the cache and return inserted geometry.
    pub fn insert_3d(&mut self, hash: HashId, geo3d: Geometry3DOutput) -> Geometry3DOutput {
        self.0
            .insert(hash, RenderCacheItem::Geometry3D(geo3d.clone()));
        geo3d
    }
}

impl Default for RenderCache {
    fn default() -> Self {
        Self::new()
    }
}
