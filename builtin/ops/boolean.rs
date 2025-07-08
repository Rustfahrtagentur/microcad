// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Built-in boolean operations.

use microcad_core::BooleanOp;
use microcad_lang::{
    eval::EvalError, model_tree::*, resolve::*, src_ref::*, syntax::*, value::Value,
};

/// Creates a node containing a boolean operation.
fn boolean_op_node(op: BooleanOp) -> Result<Value, EvalError> {
    Ok(ModelNodeBuilder::new_operation(op, SrcRef(None))
        .build()
        .into())
}

/// Creates a symbol containing a difference operation.
pub fn difference() -> Symbol {
    Symbol::new_builtin(Identifier::no_ref("difference"), None, &|_, _, _| {
        boolean_op_node(BooleanOp::Difference)
    })
}

/// Creates a symbol containing a union operation.
pub fn union() -> Symbol {
    Symbol::new_builtin(Identifier::no_ref("union"), None, &|_, _, _| {
        boolean_op_node(BooleanOp::Union)
    })
}

/// Creates a symbol containing an intersection operation.
pub fn intersection() -> Symbol {
    Symbol::new_builtin(Identifier::no_ref("intersection"), None, &|_, _, _| {
        boolean_op_node(BooleanOp::Intersection)
    })
}
