// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Render hash functionality.

use std::hash::Hasher;

use derive_more::Deref;

/// Render hash type.
pub type HashId = u64;

/// Trait to implement for typed that contain a pre-computed hash value.
pub trait ComputedHash {
    /// Return computed hash value.
    fn computed_hash(&self) -> HashId;
}

/// Generic wrapper that contains the hashed value.
#[derive(Deref, Debug, Clone)]
pub struct Hashed<T: std::hash::Hash> {
    #[deref]
    inner: T,
    hash: HashId,
}

impl<T: std::hash::Hash> Hashed<T> {
    /// Create a new wrapper with hashed.
    pub fn new(inner: T) -> Self {
        let mut hasher = std::hash::DefaultHasher::new();
        inner.hash(&mut hasher);
        Self {
            inner,
            hash: hasher.finish(),
        }
    }
}

impl<T: std::hash::Hash> ComputedHash for Hashed<T> {
    fn computed_hash(&self) -> HashId {
        self.hash
    }
}
