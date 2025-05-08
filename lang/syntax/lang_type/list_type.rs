// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! List type syntax element

use crate::ty::*;

#[allow(rustdoc::broken_intra_doc_links)]
/// List type (e.g. '[scalar]')
#[derive(Debug, Clone, PartialEq)]
pub struct ListType(Box<Type>);

impl ListType {
    /// Generate `ListType` from `Type`
    pub fn new(t: Type) -> Self {
        Self(Box::new(t))
    }
}

impl crate::ty::Ty for ListType {
    fn ty(&self) -> Type {
        self.0.as_ref().clone()
    }
}

impl std::fmt::Display for ListType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}
