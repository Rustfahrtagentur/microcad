// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Named tuple type syntax element

use crate::{syntax::*, r#type::*};

/// Named tuple (e.g. `(n: scalar, m: string)`)
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NamedTupleType(pub std::collections::BTreeMap<Identifier, Type>);

impl std::fmt::Display for NamedTupleType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(")?;
        for (i, (identifier, ty)) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", identifier, ty)?;
        }
        write!(f, ")")
    }
}
