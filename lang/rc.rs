// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Short-cut definition of Rc<std::cell::RefCell<T>> and Rc<T>

pub use std::rc::Rc;

/// Just a short cut definition
#[derive(Debug)]
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

impl<T> std::ops::Deref for RcMut<T> {
    type Target = Rc<std::cell::RefCell<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for RcMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
