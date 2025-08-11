// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A envelop of an item which is either loaded or not.

use crate::eval::*;

/// Implement this trait for linkable items
pub trait Linkable<L> {
    /// return link identifier of self
    fn link(&self) -> L;
}

/// Item which is either loaded or not loaded.
///
/// Access a loaded item via `Deref` or get a panic when accessing an unloaded item.
/// By using [`Self::load()`] unloaded items can be loaded from a *loader*.
#[derive(Clone, Debug)]
pub struct Link<T, L>(LinkInner<T, L>);

impl<T, L> Link<T, L> {
    /// Load unloaded item using given  loader function `f`.
    ///
    /// Does nothing if object was already loaded.
    pub fn load<F: FnMut(&L) -> EvalResult<T>>(&mut self, mut f: F) -> EvalResult<&T> {
        if let LinkInner::Unloaded(l) = &self.0 {
            self.0 = LinkInner::Loaded(f(l)?);
        }
        Ok(&*self)
    }
    /// Return `true` if loaded.
    pub fn is_loaded(&self) -> bool {
        match &self.0 {
            LinkInner::Unloaded(_) => false,
            LinkInner::Loaded(_) => true,
        }
    }
}

impl<T, L> std::ops::Deref for Link<T, L> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match &self.0 {
            LinkInner::Loaded(t) => t,
            LinkInner::Unloaded(_) => unreachable!("Link not loaded"),
        }
    }
}

impl<T, L> From<T> for Link<T, L> {
    fn from(t: T) -> Self {
        Self(LinkInner::Loaded(t))
    }
}

impl<T, L> serde::Serialize for Link<T, L>
where
    T: Linkable<L>,
    L: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let link = match &self.0 {
            LinkInner::Loaded(t) => &t.link(),
            LinkInner::Unloaded(l) => l,
        };
        link.serialize(serializer)
    }
}

impl<'de, T, L> serde::Deserialize<'de> for Link<T, L>
where
    L: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(LinkInner::Unloaded(L::deserialize(deserializer)?)))
    }
}

/// Either a loaded *type* or a unloaded *link*
#[derive(Clone, Debug, bincode::Encode, bincode::Decode, serde::Serialize, serde::Deserialize)]
pub enum LinkInner<T, L> {
    /// Loaded type
    Loaded(T),
    /// Unloaded link
    Unloaded(L),
}
