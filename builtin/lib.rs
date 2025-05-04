// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad builtin library

mod algorithm;
mod assert;
mod geo2d;
mod geo3d;
mod math;
mod print;

mod namespace_builder;

use microcad_lang::resolve::*;
pub use namespace_builder::NamespaceBuilder;

pub(crate) use algorithm::*;
pub(crate) use assert::*;
pub(crate) use geo2d::geo2d;
pub(crate) use geo3d::geo3d;
pub(crate) use math::math;
pub(crate) use print::print;

/// Build the standard module
pub fn builtin_namespace() -> Symbol {
    NamespaceBuilder::new("__builtin".try_into().expect("unexpected name error"))
        .symbol(assert())
        .symbol(assert_valid())
        .symbol(assert_invalid())
        .symbol(print())
        .symbol(algorithm())
        .symbol(math())
        .symbol(geo2d())
        .symbol(geo3d())
        .build()
}
