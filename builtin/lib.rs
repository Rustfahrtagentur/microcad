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

mod namespace_builder;

/// Global test initialization.
#[cfg(test)]
#[ctor::ctor]
fn init() {
    env_logger::init();
}

use microcad_lang::resolve::*;
pub use namespace_builder::NamespaceBuilder;

pub(crate) use algorithm::*;
pub(crate) use assert::*;
pub(crate) use math::math;
pub(crate) use print::print;
pub(crate) use transform::*;

/// Build the standard module
pub fn builtin_namespace() -> Symbol {
    NamespaceBuilder::new("__builtin".try_into().expect("unexpected name error"))
        .symbol(assert())
        .symbol(assert_valid())
        .symbol(assert_invalid())
        .symbol(print())
        .symbol(algorithm())
        .symbol(transform())
        .symbol(math())
        .symbol(geo2d::geo2d())
        .symbol(geo3d::geo3d())
        .build()
}
