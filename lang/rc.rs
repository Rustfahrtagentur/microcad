// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Short-cut definition of `Rc<std::cell::RefCell<T>>` and `Rc<T>`

use derive_more::{Deref, DerefMut};
pub use std::rc::Rc;

#[cfg(feature = "debug-cell")]
use debug_cell::RefCell;

#[cfg(not(feature = "debug-cell"))]
use std::cell::RefCell;

/// Just a short cut definition
#[derive(Deref, DerefMut)]
pub struct RcMut<T>(Rc<RefCell<T>>);

impl<T> RcMut<T> {
    /// Create new instance
    pub fn new(t: T) -> Self {
        Self(Rc::new(RefCell::new(t)))
    }
}

impl<T> Clone for RcMut<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> From<T> for RcMut<T> {
    fn from(value: T) -> Self {
        RcMut::new(value)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for RcMut<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RcMut").field(&self.0.borrow()).finish()
    }
}
