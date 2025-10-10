// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod circle;
mod line;
mod pie;
mod rect;
mod text;

pub use circle::*;
pub use line::*;
pub use pie::*;
pub use rect::*;
pub use text::*;

use microcad_lang::builtin::*;

/// Module for built-in 2D geometries.
pub fn geo2d() -> Symbol {
    crate::ModuleBuilder::new("geo2d".try_into().expect("valid id"))
        .symbol(Circle::symbol())
        .symbol(Line::symbol())
        .symbol(Pie::symbol())
        .symbol(Rect::symbol())
        .symbol(Text::symbol())
        .build()
}
