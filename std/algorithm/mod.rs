// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::NamespaceBuilder;
use microcad_lang::{
    builtin_module,
    eval::{EvalError, Symbols},
    parse::*,
};
use microcad_render::Node;

/// Creates a node containing a difference algorithm
pub fn difference() -> Result<Node, EvalError> {
    Ok(microcad_core::algorithm::boolean_op::difference())
}

/// Creates a node containing a union algorithm
pub fn union() -> Result<Node, EvalError> {
    Ok(microcad_core::algorithm::boolean_op::union())
}

/// Creates a node containing an intersection algorithm
pub fn intersection() -> Result<Node, EvalError> {
    Ok(microcad_core::algorithm::boolean_op::intersection())
}

/// Creates a node containing a complement algorithm
pub fn complement() -> Result<Node, EvalError> {
    Ok(microcad_core::algorithm::boolean_op::complement())
}

/// Creates the builtin `algorithm` module
pub fn builtin_module() -> std::rc::Rc<NamespaceDefinition> {
    NamespaceBuilder::new("algorithm")
        .add(builtin_module!(difference()).into())
        .add(builtin_module!(intersection()).into())
        .add(builtin_module!(union()).into())
        .add(builtin_module!(complement()).into())
        .build()
}
