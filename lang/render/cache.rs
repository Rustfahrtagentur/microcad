// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Render cache.

use crate::render::{GeometryOutput, HashId};

/// An item in the [`RenderCache`].
pub struct RenderCacheItem {
    content: GeometryOutput,
    hits: u64,
    last_access: u64,
}

impl RenderCacheItem {
    pub fn new(content: impl Into<GeometryOutput>, time_stamp: u64) -> Self {
        Self {
            content: content.into(),
            hits: 1,
            last_access: time_stamp,
        }
    }
}

/// The [`RenderCache`] owns all geometry created during the render process.
pub struct RenderCache {
    current_cycle: u64,
    items: rustc_hash::FxHashMap<HashId, RenderCacheItem>,
}

impl RenderCache {
    /// Create a new empty cache.
    pub fn new() -> Self {
        Self {
            current_cycle: 0,
            items: Default::default(),
        }
    }

    /// Garbage collection.
    pub fn gc(&mut self) {
        self.current_cycle += 1;
        self.sweep(16);
    }

    /// Empty cache entirely.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Remove old items from the cache.
    fn sweep(&mut self, cost: u64) {
        let old_count = self.items.len();
        self.items
            .retain(|_, item| (self.current_cycle - item.last_access) / item.hits <= cost);
        let removed = old_count - self.items.len();
        log::trace!("Removed {removed} items from cache.")
    }

    /// Get geometry output from the cache.
    pub fn get(&mut self, hash: &HashId) -> Option<&GeometryOutput> {
        match self.items.get_mut(hash) {
            Some(output) => {
                output.hits += 1;
                output.last_access = self.current_cycle;
                log::trace!("Cache hit: {hash:X}");
                Some(&output.content)
            }
            _ => None,
        }
    }

    /// Insert geometry output into the cache and return inserted geometry.
    pub fn insert(
        &mut self,
        hash: impl Into<HashId>,
        geo: impl Into<GeometryOutput>,
    ) -> &GeometryOutput {
        let hash: HashId = hash.into();
        self.items
            .insert(hash, RenderCacheItem::new(geo, self.current_cycle));
        self.get(&hash).expect("Cached item")
    }
}

impl Default for RenderCache {
    fn default() -> Self {
        Self::new()
    }
}
