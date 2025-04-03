// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluation of parsed content

mod argument_map;
mod body;
mod builtin_function;
mod builtin_module;
mod call;
mod eval_context;
mod eval_error;
mod expression;
mod format_string;
mod identifier;
mod literal;
mod output;
mod parameter;
mod scope_stack;
mod statement;
mod r#use;

pub use argument_map::*;
pub use builtin_function::*;
pub use builtin_module::*;
pub use call::*;
pub use eval_context::*;
pub use eval_error::*;
pub use output::*;

use crate::{diag::*, resolve::*, src_ref::*, syntax::*, ty::*, value::*};
use scope_stack::*;

/// Evaluation trait
pub trait Eval {
    /// Evaluate the type into an expression
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value>;
}

impl MethodCall {
    /// Evaluate method call
    fn eval(&self, _context: &mut EvalContext, _lhs: &Box<Expression>) -> EvalResult<Value> {
        todo!()
    }
}
