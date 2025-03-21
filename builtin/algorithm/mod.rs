// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::{objects::*, SymbolNode, *};

/// Creates a node containing a difference algorithm
pub fn difference() -> RcMut<SymbolNode> {
    SymbolNode::new_builtin_module("difference", &|_, _| Ok(algorithm::difference()))
}

/// Creates a node containing a union algorithm
pub fn union() -> RcMut<SymbolNode> {
    SymbolNode::new_builtin_module("union", &|_, _| Ok(algorithm::union()))
}

/// Creates a node containing an intersection algorithm
pub fn intersection() -> RcMut<SymbolNode> {
    SymbolNode::new_builtin_module("intersection", &|_, _| Ok(algorithm::intersection()))
}

/// Creates a node containing a complement algorithm
pub fn complement() -> RcMut<SymbolNode> {
    SymbolNode::new_builtin_module("complement", &|_, _| Ok(algorithm::complement()))
}

/// Creates the builtin `algorithm` module
pub fn build() -> RcMut<SymbolNode> {
    let mut ns = SymbolNode::new_builtin_namespace("algorithm");
    SymbolNode::insert_child(&mut ns, difference());
    SymbolNode::insert_child(&mut ns, union());
    SymbolNode::insert_child(&mut ns, intersection());
    SymbolNode::insert_child(&mut ns, complement());
    ns
}
