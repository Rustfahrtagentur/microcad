// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::{objects::*, resolve::*, syntax::*, value::*};

/// Creates a node containing a difference algorithm
fn difference() -> Symbol {
    Symbol::new_builtin(Identifier::no_ref("difference"), &|_, _| {
        Ok(Value::Node(algorithm::difference()))
    })
}

/// Creates a node containing a union algorithm
fn union() -> Symbol {
    Symbol::new_builtin(Identifier::no_ref("union"), &|_, _| {
        Ok(Value::Node(algorithm::union()))
    })
}

/// Creates a node containing an intersection algorithm
fn intersection() -> Symbol {
    Symbol::new_builtin(Identifier::no_ref("intersection"), &|_, _| {
        Ok(Value::Node(algorithm::intersection()))
    })
}

/// Creates a node containing a complement algorithm
fn complement() -> Symbol {
    Symbol::new_builtin(Identifier::no_ref("complement"), &|_, _| {
        Ok(Value::Node(algorithm::complement()))
    })
}

/// Creates the builtin `algorithm` namespace
pub fn algorithm() -> Symbol {
    crate::NamespaceBuilder::new("algorithm".try_into().expect("valid id"))
        .symbol(difference())
        .symbol(union())
        .symbol(intersection())
        .symbol(complement())
        .build()
}
