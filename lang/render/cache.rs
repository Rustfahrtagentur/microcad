// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Render cache.

use std::{
    hash::{Hash, Hasher},
    rc::Rc,
};

use microcad_core::{Geometry2D, Geometry3D};

/// Render hash type.
#[derive(PartialEq, Eq, Default)]
pub struct RenderHash(u64);

impl Hash for RenderHash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.0);
    }
}

/*pub trait RenderHashable: fmt::Display {
    fn hash(&self) -> RenderHash {
        let mut hasher = DefaultHasher::new();
        self.to_string().hash(&mut hasher);
        RenderHash(hasher.finish())
    }
}

pub struct RenderHashed<T: RenderHashable> {
    inner: T,
    hash: RenderHash,
}

impl<T: RenderHashable> RenderHashed<T> {
    pub fn new(inner: T) -> Self {
        let hash = inner.hash();
        Self { inner, hash }
    }
}*/

/// An item in the [`RenderCache`].
pub enum RenderCacheItem {
    /// 2D geometry. Note: The Rc can be removed eventually, once the implementation of RenderHash is finished.
    Geometry2D(Rc<Geometry2D>),
    /// 3D geometry. Note: The Rc can be removed eventually, once the implementation of RenderHash is finished.
    Geometry3D(Rc<Geometry3D>),
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
    pub fn get_2d(&self, hash: &RenderHash) -> Option<&Rc<Geometry2D>> {
        match self.0.get(hash) {
            Some(RenderCacheItem::Geometry2D(g)) => Some(g),
            _ => None,
        }
    }

    /// Get 3D geometry from the cache.
    pub fn get_3d(&self, hash: &RenderHash) -> Option<&Rc<Geometry3D>> {
        match self.0.get(hash) {
            Some(RenderCacheItem::Geometry3D(g)) => Some(g),
            _ => None,
        }
    }

    /// Insert 2D geometry into the cache and return inserted geometry.
    pub fn insert_2d(&mut self, hash: RenderHash, geo2d: Geometry2D) -> Rc<Geometry2D> {
        let geo2d = Rc::new(geo2d);
        self.0
            .insert(hash, RenderCacheItem::Geometry2D(geo2d.clone()));
        geo2d
    }

    /// Insert 3D geometry into the cache and return inserted geometry.
    pub fn insert_3d(&mut self, hash: RenderHash, geo3d: Geometry3D) -> Rc<Geometry3D> {
        let geo3d = Rc::new(geo3d);
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
