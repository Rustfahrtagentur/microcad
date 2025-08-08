// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Type annotation

use crate::{src_ref::*, syntax::*, ty::*};

/// Type within source code
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TypeAnnotation(pub Refer<Type>);

impl SrcReferrer for TypeAnnotation {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

impl std::fmt::Display for TypeAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl crate::ty::Ty for TypeAnnotation {
    fn ty(&self) -> Type {
        self.0.value.clone()
    }
}

impl TreeDisplay for TypeAnnotation {
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeIndent) -> std::fmt::Result {
        writeln!(f, "{:depth$}TypeAnnotation: {}", "", self.0.value)
    }
}
