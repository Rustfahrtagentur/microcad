// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod circle;
mod rect;

pub use circle::*;
pub use rect::*;

use microcad_lang::{eval::*, resolve::*};

/// Builtin module for 2D geometry
pub fn geo2d() -> Symbol {
    crate::ModuleBuilder::new("geo2d".try_into().expect("valid id"))
        .symbol(Circle::symbol())
        .symbol(Rect::symbol())
        .build()
}
