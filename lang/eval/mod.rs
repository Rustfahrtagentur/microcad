// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluation of parsed content.
//!
//! To be able to evaluate (run) a source file, it must be loaded, parsed and resolved.
//! To do so a [`Context`] can be created with [`Context::new()`] based on an already resolved symbol or
//! by using [`Context::from_source()`] or [`Context::from_source_captured()`] which both automatically
//! load and resolve the source file and build a context around it which then can be evaluated with [`Context::eval()`]:
//!
//! ```
//! use microcad_builtin::builtin_namespace;
//! use microcad_lang::eval::Context;
//! use std::io::stdout;
//!
//! // create a context for evaluation of the source file
//! let mut context = Context::from_source(
//!     "my.µcad",              // root file name
//!     builtin_namespace(),    // `__builtin` library
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
mod body;
mod builtin;
mod call;
mod call_stack;
mod context;
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
pub use builtin::*;
pub use call::*;
pub use call_stack::*;
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
    /// Evaluate method call
    fn eval(&self, context: &mut Context, _lhs: &Expression) -> EvalResult<Value> {
        context.error(
            self,
            EvalError::Todo(format!(
                "cannot evaluate method call of {} at {}",
                self,
                context.locate(self)?
            )),
        )?;
        Ok(Value::None)
    }
}
