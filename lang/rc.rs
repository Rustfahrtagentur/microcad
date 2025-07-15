// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Short-cut definition of `Rc<std::cell::RefCell<T>>` and `Rc<T>`

use derive_more::{Deref, DerefMut};
pub use std::rc::Rc;

/// Just a short cut definition
#[derive(Debug, Deref, DerefMut)]
pub struct RcMut<T>(Rc<std::cell::RefCell<T>>);

impl<T> RcMut<T> {
    /// Create new instance
    pub fn new(t: T) -> Self {
        Self(Rc::new(std::cell::RefCell::new(t)))
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
