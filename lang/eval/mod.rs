// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluation of parsed content

mod argument_map;
mod body;
mod builtin_function;
mod builtin_module;
mod call;
mod call_stack;
mod eval_context;
mod eval_error;
mod expression;
mod externals;
mod format_string;
mod identifier;
mod literal;
mod local_stack;
mod output;
mod parameter;
mod source_cache;
mod statement;
mod r#use;

pub use argument_map::*;
pub use builtin_function::*;
pub use builtin_module::*;
pub use call::*;
pub use eval_context::*;
pub use eval_error::*;
pub use externals::*;
pub use output::*;
pub use source_cache::*;

use crate::{diag::*, resolve::*, src_ref::*, syntax::*, ty::*, value::*};
use local_stack::*;

/// Evaluation trait
pub trait Eval {
    /// Evaluate the type into an expression
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value>;
}

impl MethodCall {
    /// Evaluate method call
    fn eval(&self, _context: &mut EvalContext, _lhs: &Expression) -> EvalResult<Value> {
        todo!()
    }
}
