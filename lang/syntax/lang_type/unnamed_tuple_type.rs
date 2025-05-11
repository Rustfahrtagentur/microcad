// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unnamed tuple syntax element

use crate::ty::*;

/// Unnamed tuple type (e.g. `(scalar,string)`
#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct UnnamedTupleType(pub Vec<Type>);

impl std::fmt::Display for UnnamedTupleType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(")?;
        for (i, t) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", t)?;
        }
        write!(f, ")")
    }
}
