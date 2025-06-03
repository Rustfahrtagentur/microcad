// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod rotate;
mod translate;

pub use rotate::*;
pub use translate::*;

use microcad_lang::{eval::*, resolve::*};

/// Builtin namespace for 2D geometry
pub fn transform() -> Symbol {
    crate::NamespaceBuilder::new("transform".try_into().expect("valid id"))
        .symbol(Translate::symbol())
        .symbol(Rotate::symbol())
        .build()
}
