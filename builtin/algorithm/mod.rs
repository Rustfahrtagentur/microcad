// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::NamespaceBuilder;
use microcad_lang::{builtin_module, eval::*, objects::*, parse::*};

/// Creates a node containing a difference algorithm
pub fn difference() -> Result<ObjectNode, EvalError> {
    Ok(algorithm::difference())
}

/// Creates a node containing a union algorithm
pub fn union() -> Result<ObjectNode, EvalError> {
    Ok(algorithm::union())
}

/// Creates a node containing an intersection algorithm
pub fn intersection() -> Result<ObjectNode, EvalError> {
    Ok(algorithm::intersection())
}

/// Creates a node containing a complement algorithm
pub fn complement() -> Result<ObjectNode, EvalError> {
    Ok(algorithm::complement())
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
