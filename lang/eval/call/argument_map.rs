// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Argument map evaluation entity

use crate::{eval::*, parse::Combinations, src_ref::*};

/// Map of arguments
#[derive(Clone, Debug, Default)]
pub struct ArgumentMap(Refer<std::collections::HashMap<Id, Value>>);

impl ArgumentMap {
    /// Create empty argument map
    pub fn new(src_ref: SrcRef) -> Self {
        Self(Refer::new(std::collections::HashMap::new(), src_ref))
    }

    /// Fetch an argument by name
    pub fn get_value<'a, T>(&'a self, name: &str) -> T
    where
        T: std::convert::TryFrom<&'a Value>,
        T::Error: std::fmt::Debug,
    {
        self.0
            .get(name)
            .expect("no name found")
            .try_into()
            .expect("cannot convert argument value")
    }
}

impl SrcReferrer for ArgumentMap {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

impl std::ops::Deref for ArgumentMap {
    type Target = std::collections::HashMap<Id, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ArgumentMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default)]
pub struct MultiArgumentMap(crate::parse::call::CombinationMap<Value>);

impl MultiArgumentMap {
    /// Insert a multi-value coefficient
    pub fn insert_multi(&mut self, name: Id, value: Vec<Value>) {
        self.0
            .insert(name, crate::parse::call::Coefficient::Multi(value));
    }

    /// Insert a single-value coefficient
    pub fn insert_single(&mut self, name: Id, value: Value) {
        self.0
            .insert(name, crate::parse::call::Coefficient::Single(value));
    }

    /// Return an iterator over all combinations
    pub fn combinations(&self) -> Combinations<Value> {
        Combinations::new(&self.0)
    }

    /// Check if the argument map contains a key
    pub fn contains_key(&self, key: &Id) -> bool {
        self.0.contains_key(key)
    }
}
