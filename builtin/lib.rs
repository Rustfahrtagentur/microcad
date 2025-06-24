// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad builtin library

mod algorithm;
mod assert;
mod geo2d;
mod geo3d;
mod math;
mod print;
mod transform;

/// Global test initialization.
#[cfg(test)]
#[ctor::ctor]
fn init() {
    env_logger::init();
}

pub use microcad_lang::builtin::*;
use microcad_lang::resolve::*;

pub(crate) use algorithm::*;
pub(crate) use assert::*;
pub(crate) use math::*;
pub(crate) use print::*;
pub(crate) use transform::*;

/// Build the standard module
pub fn builtin_module() -> Symbol {
    ModuleBuilder::new("__builtin".try_into().expect("unexpected name error"))
        .symbol(assert())
        .symbol(assert_valid())
        .symbol(assert_invalid())
        .symbol(print())
        .symbol(error())
        .symbol(warning())
        .symbol(algorithm())
        .symbol(transform())
        .symbol(math())
        .symbol(geo2d::geo2d())
        .symbol(geo3d::geo3d())
        .build()
}
