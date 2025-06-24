// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::BooleanOp;
use microcad_lang::{model_tree::*, resolve::*, src_ref::*, syntax::*, value::*};

/// Creates a node containing a difference operation
fn difference() -> Symbol {
    Symbol::new_builtin(Identifier::no_ref("difference"), None, &|_, _, _| {
        Ok(Value::from_single_node(ModelNode::new_operation(
            BooleanOp::Difference,
            SrcRef(None),
        )))
    })
}

/// Creates a node containing a union operation
fn union() -> Symbol {
    Symbol::new_builtin(Identifier::no_ref("union"), None, &|_, _, _| {
        Ok(Value::from_single_node(ModelNode::new_operation(
            BooleanOp::Union,
            SrcRef(None),
        )))
    })
}

/// Creates a node containing an intersection operation
fn intersection() -> Symbol {
    Symbol::new_builtin(Identifier::no_ref("intersection"), None, &|_, _, _| {
        Ok(Value::from_single_node(ModelNode::new_operation(
            BooleanOp::Intersection,
            SrcRef(None),
        )))
    })
}

/// Creates a node containing a complement operation
fn complement() -> Symbol {
    Symbol::new_builtin(Identifier::no_ref("complement"), None, &|_, _, _| {
        Ok(Value::from_single_node(ModelNode::new_operation(
            BooleanOp::Complement,
            SrcRef(None),
        )))
    })
}

/// Creates the builtin `operation` module
pub fn ops() -> Symbol {
    crate::ModuleBuilder::new("ops".try_into().expect("valid id"))
        .symbol(difference())
        .symbol(union())
        .symbol(intersection())
        .symbol(complement())
        .build()
}
