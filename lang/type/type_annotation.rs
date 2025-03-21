// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Basic Types

use super::Type;
use crate::{src_ref::*, syntax::*};

/// Type within source code
#[derive(Debug, Clone, PartialEq)]
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

impl From<Type> for TypeAnnotation {
    fn from(value: Type) -> Self {
        TypeAnnotation(Refer::none(value))
    }
}

impl PrintSyntax for TypeAnnotation {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$} TypeAnnotation: {}", "", self.0.value)
    }
}
