// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::NamespaceBuilder;
use microcad_lang::{
    builtin_module,
    eval::{EvalError, Symbols},
    parse::*,
};
use microcad_render::Node;

pub fn difference() -> Result<Node, EvalError> {
    Ok(microcad_core::algorithm::boolean_op::difference())
}

pub fn union() -> Result<Node, EvalError> {
    Ok(microcad_core::algorithm::boolean_op::union())
}

pub fn intersection() -> Result<Node, EvalError> {
    Ok(microcad_core::algorithm::boolean_op::intersection())
}

pub fn xor() -> Result<Node, EvalError> {
    Ok(microcad_core::algorithm::boolean_op::xor())
}

pub fn builtin_module() -> std::rc::Rc<ModuleDefinition> {
    NamespaceBuilder::new("algorithm")
        .add_builtin_module(builtin_module!(difference()))
        .add_builtin_module(builtin_module!(intersection()))
        .add_builtin_module(builtin_module!(union()))
        .add_builtin_module(builtin_module!(xor()))
        .build()
}

