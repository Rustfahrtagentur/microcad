// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::{objects::*, resolve::*};

/// Creates a node containing a difference algorithm
fn difference() -> SymbolNode {
    SymbolNode::new_builtin_module("difference", &|_, _| Ok(algorithm::difference()))
}

/// Creates a node containing a union algorithm
fn union() -> SymbolNode {
    SymbolNode::new_builtin_module("union", &|_, _| Ok(algorithm::union()))
}

/// Creates a node containing an intersection algorithm
fn intersection() -> SymbolNode {
    SymbolNode::new_builtin_module("intersection", &|_, _| Ok(algorithm::intersection()))
}

/// Creates a node containing a complement algorithm
fn complement() -> SymbolNode {
    SymbolNode::new_builtin_module("complement", &|_, _| Ok(algorithm::complement()))
}

/// Creates the builtin `algorithm` module
pub fn algorithm() -> SymbolNode {
    crate::NamespaceBuilder::new("algorithm".try_into().expect("unexpected name error"))
        .symbol(difference())
        .symbol(union())
        .symbol(intersection())
        .symbol(complement())
        .build()
}
