// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluation of parsed content.
//!
//! To be able to evaluate (run) a source file, it must be loaded, parsed and resolved.
//! To do so a [`Context`] can be created with [`Context::new()`] based on an already resolved symbol or
//! by using [`Context::from_source()`] or [`Context::from_source_captured()`] which both automatically
//! load and resolve the source file and build a context around it which then can be evaluated with [`Context::eval()`]:
//!
//! ```
//! use microcad_builtin::builtin_module;
//! use microcad_lang::eval::Context;
//! use std::io::stdout;
//!
//! // create a context for evaluation of the source file
//! let mut context = Context::from_source(
//!     "my.µcad",              // root file name
//!     builtin_module(),    // `__builtin` library
//!     &["./lib".into()]       // list of library paths
//! ).expect("successful load, parse and resolve");
//!
//! // evaluate the source file in it's context
//! let value = context.eval().expect("successful evaluation");
//!
//! // print any error
//! context.write_diagnosis(stdout()).expect("stdout should be available");
//! ```

mod argument_map;
mod attribute;
mod body;
mod builtin;
mod call;
mod context;
mod eval_error;
mod expression;
mod externals;
mod format_string;
mod function;
mod init;
mod literal;
mod output;
mod parameter;
mod part;
mod source_file;
mod statement;
mod symbols;
mod tuple;
mod r#use;

pub use argument_map::*;
pub use attribute::*;
pub use builtin::*;
pub use call::*;
pub use context::*;
pub use eval_error::*;
pub use externals::*;
pub use output::*;
pub use r#use::*;
pub use symbols::*;

use crate::{diag::*, resolve::*, src_ref::*, syntax::*, ty::*, value::*};

/// Evaluation trait
pub trait Eval {
    /// Evaluate the type into an expression
    fn eval(&self, context: &mut Context) -> EvalResult<Value>;
}

impl MethodCall {
    /// Evaluate method call.
    ///
    /// Examples:
    /// ```microcad
    /// assert([2.0, 2.0].all_equal(), "All elements in this list must be equal.");
    /// ```
    fn eval(&self, context: &mut Context, lhs: &Expression) -> EvalResult<Value> {
        lhs.eval(context)?
            .call_method(&self.id, &self.argument_list, context)
    }
}
