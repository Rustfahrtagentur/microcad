// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Object attributes module
//!
use crate::{syntax::*, value::*};
/// Node metadata, from an evaluated attribute list.
#[derive(Clone, Debug, Default)]
pub struct Metadata(NamedTuple);

impl Metadata {
    /// Create new meta data from map.
    pub fn new(map: std::collections::BTreeMap<Identifier, Value>) -> Self {
        Self(NamedTuple::new(map))
    }
}

impl std::ops::Deref for Metadata {
    type Target = NamedTuple;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Metadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
