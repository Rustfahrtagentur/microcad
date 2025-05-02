// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluation of parsed content.
//!
//! To be able to evaluate (run) a source file, it must be loaded, parsed and resolved.
//! To do so an [`EvalContext`] can be created with [`EvalContext::new()`] based on a already resolved symbol or
//! by using [`EvalContext::from_source()`] or [`EvalContext::from_source_captured()`] which automatically
//! load and resolve the source file and build a context around it which then can be evaluated with [`EvalContext::eval()`]:
//!
//! ```
//! use microcad_builtin::builtin_namespace;
//! use microcad_lang::eval::EvalContext;
//! use std::io::stdout;
//!
//! // create a context for evaluation of the source file
//! let mut context = EvalContext::from_source(
//!     "my.µcad",              // root file name
//!     builtin_namespace(),    // `__builtin` library
//!     &["./lib".into()]       // list of library paths
//! );
//!
//! // evaluate the source file in it's context
//! let value = context.eval().expect("evaluation success");
//!
//! // print any error
//! context.pretty_print( stdout(), &context).expect("UTF-8 compatible output");
//! ```

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
mod literal;
mod output;
mod parameter;
mod source_file;
mod statement;
mod symbols;
mod r#use;

pub use argument_map::*;
pub use builtin_function::*;
pub use builtin_module::*;
pub use call::*;
pub use call_stack::*;
pub use eval_context::*;
pub use eval_error::*;
pub use externals::*;
pub use output::*;
pub use r#use::*;
pub use symbols::*;

use crate::{diag::*, resolve::*, src_ref::*, syntax::*, ty::*, value::*};

/// Evaluation trait
pub trait Eval {
    /// Evaluate the type into an expression
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value>;
}

impl MethodCall {
    /// Evaluate method call
    fn eval(&self, _context: &mut EvalContext, _lhs: &Expression) -> EvalResult<Value> {
        todo!("method call not implemented")
    }
}
