// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad builtin library

mod algorithm;
mod assert;
mod math;
mod print;

mod namespace_builder;

use microcad_lang::*;
pub use namespace_builder::NamespaceBuilder;

pub(crate) use algorithm::*;
pub(crate) use assert::*;
pub(crate) use math::math;
pub(crate) use print::print;

/// Build the standard module
pub fn builtin_namespace() -> RcMut<SymbolNode> {
    NamespaceBuilder::new("__builtin")
        .symbol(assert())
        .symbol(assert_valid())
        .symbol(assert_invalid())
        .symbol(print())
        .symbol(algorithm())
        .symbol(math())
        .build()
}
