// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod cube;
mod cylinder;
mod sphere;

pub use cube::*;
pub use cylinder::*;
pub use sphere::*;

use microcad_lang::{eval::*, resolve::*};

/// geo3d Builtin module
pub fn geo3d() -> Symbol {
    crate::ModuleBuilder::new("geo3d".try_into().expect("valid id"))
        .symbol(Sphere::symbol())
        .symbol(Cube::symbol())
        .symbol(Cylinder::symbol())
        .build()
}
