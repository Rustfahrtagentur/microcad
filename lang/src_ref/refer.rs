// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::src_ref::*;
use derive_more::{Deref, DerefMut};

/// Packs any value together with a source reference
#[derive(Clone, Default, Ord, PartialEq, PartialOrd, Deref, DerefMut)]
pub struct Refer<T> {
    /// Value
    #[deref]
    #[deref_mut]
    pub value: T,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl<T> Refer<T> {
    /// Create a `Refer` instance without source code reference
    pub fn none(value: T) -> Self {
        Self {
            value,
            src_ref: SrcRef(None),
        }
    }
    /// Create a `Refer` instance with source code reference
    pub fn new(value: T, src_ref: SrcRef) -> Self {
        Self { value, src_ref }
    }
    /// Create a `Refer` instance with two source code references
    #[cfg(test)]
    pub fn merge<U, V>(left: Refer<U>, right: Refer<V>, f: fn(U, V) -> T) -> Self {
        Self {
            value: f(left.value, right.value),
            src_ref: SrcRef::merge(&left.src_ref, &right.src_ref),
        }
    }
    /// Map a `Refer` instance to a new one
    pub fn map<U>(self, f: fn(T) -> U) -> Refer<U> {
        Refer::<U> {
            value: f(self.value),
            src_ref: self.src_ref,
        }
    }
}

impl<T> SrcReferrer for Refer<T> {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl<T: Eq> Eq for Refer<T> {}

impl<T: std::fmt::Display> std::fmt::Display for Refer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.value.fmt(f)
    }
}
impl<T: std::fmt::Debug> std::fmt::Debug for Refer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}, Refer: {:?}", self.value, self.src_ref)
    }
}

impl<T: std::hash::Hash> std::hash::Hash for Refer<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T: std::iter::IntoIterator> std::iter::IntoIterator for Refer<T> {
    type Item = T::Item;
    type IntoIter = T::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}
