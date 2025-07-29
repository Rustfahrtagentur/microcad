// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Argument value evaluation entity

use crate::{src_ref::*, ty::*, value::*};

/// Argument value.
#[derive(Clone, Debug)]
pub struct ArgumentValue {
    /// *value* of the argument.
    pub value: Value,
    /// Source code reference.
    src_ref: SrcRef,
}

impl SrcReferrer for ArgumentValue {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for ArgumentValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ({})", self.value, self.value.ty())
    }
}

impl Ty for ArgumentValue {
    fn ty(&self) -> Type {
        self.value.ty()
    }
}

impl ArgumentValue {
    /// Create new argument value
    pub fn new(value: Value, src_ref: SrcRef) -> Self {
        Self { value, src_ref }
    }

    /// If argument is an array returns the inner type
    pub fn ty_inner(&self) -> Type {
        if let Type::Array(ty) = self.ty() {
            ty.ty()
        } else {
            Type::Invalid
        }
    }
}
