// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::BooleanOp;
use microcad_lang::{model_tree::*, resolve::*, src_ref::*, syntax::*, value::*};

/// Creates a node containing a difference algorithm
fn difference() -> Symbol {
    Symbol::new_builtin(Identifier::no_ref("difference"), &|_, _| {
        Ok(Value::from_single_node(ModelNode::new_transformation(
            BooleanOp::Difference,
            SrcRef(None),
        )))
    })
}

/// Creates a node containing a union algorithm
fn union() -> Symbol {
    Symbol::new_builtin(Identifier::no_ref("union"), &|_, _| {
        Ok(Value::from_single_node(ModelNode::new_transformation(
            BooleanOp::Union,
            SrcRef(None),
        )))
    })
}

/// Creates a node containing an intersection algorithm
fn intersection() -> Symbol {
    Symbol::new_builtin(Identifier::no_ref("intersection"), &|_, _| {
        Ok(Value::from_single_node(ModelNode::new_transformation(
            BooleanOp::Intersection,
            SrcRef(None),
        )))
    })
}

/// Creates a node containing a complement algorithm
fn complement() -> Symbol {
    Symbol::new_builtin(Identifier::no_ref("complement"), &|_, _| {
        Ok(Value::from_single_node(ModelNode::new_transformation(
            BooleanOp::Complement,
            SrcRef(None),
        )))
    })
}

/// Creates the builtin `algorithm` module
pub fn algorithm() -> Symbol {
    crate::ModuleBuilder::new("algorithm".try_into().expect("valid id"))
        .symbol(difference())
        .symbol(union())
        .symbol(intersection())
        .symbol(complement())
        .build()
}
