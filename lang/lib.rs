// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad source code parser

pub mod argument_map;
pub mod diag;
pub mod eval;
pub mod objects;
pub mod ord_map;
pub mod parse;
pub mod parser;
pub mod resolve;
pub mod source_file_cache;
pub mod src_ref;
pub mod ty;
pub mod r#type;
pub mod value;

use std::rc::Rc;

#[derive(Debug)]
pub struct RcMut<T>(Rc<std::cell::RefCell<T>>);

impl<T> RcMut<T> {
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

use std::sync::Once;

pub use value::*;

/// Id type (base of all identifiers)
pub type Id = compact_str::CompactString;

static INIT_EVENT_LOGGER: Once = Once::new();

/// Initialize env_logger
pub fn env_logger_init() {
    INIT_EVENT_LOGGER.call_once(env_logger::init);
}
