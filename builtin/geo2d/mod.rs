// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod circle;
mod line;
mod rect;

pub use circle::*;
pub use line::*;
pub use rect::*;

use microcad_lang::{eval::*, resolve::*};

/// Module for built-in 2D geometries.
pub fn geo2d() -> Symbol {
    crate::ModuleBuilder::new("geo2d".try_into().expect("valid id"))
        .symbol(Circle::symbol())
        .symbol(Rect::symbol())
        .symbol(Line::symbol())
        .build()
}
