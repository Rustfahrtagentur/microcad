// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::{eval::BuiltinWorkbenchDefinition, resolve::*};

mod boolean;
mod extrude;
mod hull;
mod rotate;
mod translate;

/// Creates the builtin `operation` module
pub fn ops() -> Symbol {
    crate::ModuleBuilder::new("ops".try_into().expect("valid id"))
        .symbol(boolean::difference())
        .symbol(boolean::union())
        .symbol(boolean::intersection())
        .symbol(hull::hull())
        .symbol(translate::Translate::symbol())
        .symbol(rotate::Rotate::symbol())
        .build()
}
