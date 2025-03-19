// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluation of parsed content

mod builtin_function;
mod builtin_module;
mod call;
mod eval_context;
mod eval_error;

pub use builtin_function::*;
pub use builtin_module::*;
pub use call::*;
pub use eval_context::*;
pub use eval_error::*;

use crate::{objects::ObjectNode, Value};

pub enum EvalReturn {
    None,
    ObjectNode(ObjectNode),
    Value(Value),
}

/// Evaluation trait
pub trait Eval {
    /// Evaluate the type into an expression
    fn eval(&self, context: &mut EvalContext) -> EvalResult<EvalReturn>;
}
