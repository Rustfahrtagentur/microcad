// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Ordered Map

use std::ops::Index;

/// Trait a value in an `OrdMap` must implement.
/// # Types
/// `K`: key type
pub trait OrdMapValue<K>
where
    K: std::cmp::Eq + std::hash::Hash + Clone,
{
    /// return some unique key of this value or `None`
    fn key(&self) -> Option<K>;
}

/// Map whose values can be accessed via index in original insert order.
#[derive(Clone)]
pub struct OrdMap<K, V>
where
    V: OrdMapValue<K>,
    K: std::cmp::Eq + std::hash::Hash + Clone,
{
    /// vec to store values
    vec: Vec<V>,
    /// map to store key -> index of value in vec
    map: std::collections::HashMap<K, usize>,
}

impl<K, V> Default for OrdMap<K, V>
where
    V: OrdMapValue<K>,
    K: std::cmp::Eq + std::hash::Hash + Clone,
{
    fn default() -> Self {
        Self {
            vec: Default::default(),
            map: Default::default(),
        }
    }
}

impl<K, V> std::fmt::Debug for OrdMap<K, V>
where
    V: OrdMapValue<K> + std::fmt::Debug,
    K: std::cmp::Eq + std::hash::Hash + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OrdMap").field("vec", &self.vec).finish()
    }
}

impl<K, V> From<Vec<V>> for OrdMap<K, V>
where
    V: OrdMapValue<K>,
    K: std::cmp::Eq + std::hash::Hash + Clone,
{
    fn from(vec: Vec<V>) -> Self {
        let mut map = std::collections::HashMap::new();
        // TODO remove for loop use for_each and filter
        for (i, item) in vec.iter().enumerate() {
            if let Some(key) = item.key() {
                map.insert(key, i);
            }
        }

        Self { vec, map }
    }
}

impl<K, V> OrdMap<K, V>
where
    V: OrdMapValue<K>,
    K: std::cmp::Eq + std::hash::Hash + Clone,
{
    /// get iterator over values in original order
    pub fn iter(&self) -> std::slice::Iter<'_, V> {
        self.vec.iter()
    }

    /// return number of stored values
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// `true` no values are stored`
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    /// add new value
    pub fn try_push(&mut self, item: V) -> Result<(), K> {
        if let Some(key) = item.key().clone() {
            if self.map.contains_key(&key) {
                return Err(key);
            }
            self.map.insert(key, self.vec.len());
        }
        self.vec.push(item);
        Ok(())
    }

    /// get value by key
    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key).map(|index| &self.vec[*index])
    }

    /// get list of all keys
    pub fn keys(&self) -> std::collections::hash_map::Keys<'_, K, usize> {
        self.map.keys()
    }

    /// get first element
    pub fn first(&self) -> Option<&V> {
        self.vec.first()
    }
}

impl<K, V> Index<usize> for OrdMap<K, V>
where
    V: OrdMapValue<K>,
    K: std::cmp::Eq + std::hash::Hash + Clone,
{
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}

impl<K, V> Index<&K> for OrdMap<K, V>
where
    V: OrdMapValue<K>,
    K: std::cmp::Eq + std::hash::Hash + Clone,
{
    type Output = V;

    fn index(&self, key: &K) -> &Self::Output {
        &self.vec[*self.map.get(key).expect("key not found")]
    }
}
