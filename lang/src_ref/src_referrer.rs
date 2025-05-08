// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Trait to access source reference of an element

use crate::src_ref::*;

/// Elements holding a source code reference shall implement this trait
pub trait SrcReferrer {
    /// return source code reference
    fn src_ref(&self) -> SrcRef;
}

/// We want to be able to use SrcRef directly in functions with `impl SrcReferrer` argument
impl SrcReferrer for SrcRef {
    fn src_ref(&self) -> SrcRef {
        self.clone()
    }
}

/// We want to be able to use type references as well
impl<T: SrcReferrer> SrcReferrer for &T {
    fn src_ref(&self) -> SrcRef {
        (*self).src_ref()
    }
}
