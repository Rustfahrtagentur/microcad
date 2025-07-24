// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluation of parsed content.
//!
//! To be able to evaluate (run) a source file, it must be loaded, parsed and resolved.
//! To do so a [`Context`] can be created with [`Context::new()`] based on an already resolved symbol or
//! by using [`Context::from_source()`] or [`Context::from_source_captured()`] which both automatically
//! load and resolve the source file and build a context around it which then can be evaluated with [`Context::eval()`]:
//!
//! ```ignore
//! use microcad_lang::eval::Context;
//! use microcad_lang::diag::Diag;
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
//! let node = context.eval().expect("successful evaluation");
//!
//! // print any error
//! println!("{}", context.diagnosis());
//! ```

mod argument_match;
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
mod module_definition;
mod output;
mod parameter;
mod source_file;
mod statements;
mod symbols;
mod tuple;
mod workbench;

pub use argument_match::*;
pub use attribute::*;
pub use builtin::*;
pub use call::*;
pub use context::*;
pub use eval_error::*;
pub use externals::*;
pub use output::*;
pub use parameter::*;
pub use statements::*;
pub use symbols::*;

use crate::{diag::*, resolve::*, src_ref::*, syntax::*, ty::*, value::*};

/// Evaluation trait.
///
/// The return type `T` defines to which output type the type is evaluated.
/// Usually, these output types are used in specific context:
///
/// | Return type `T`     | Context / Scope                                     | Return value on error   | Description                                                             |
/// | ------------------- | --------------------------------------------------- | ----------------------- | ----------------------------------------------------------------------- |
/// | `()`                | [`Assignment`].                                     | `()`                    | An assignment returns nothing but alters the symbol table.              |
/// |                     |                                                     |                         |
/// | `Value`             | Function calls, module statements,                  | `Value::None`           | These trait implementations are   
/// |                     | parameter lists, argument lists.                    |                         | mostly used when evaluating functions.
/// |                     |                                                     |                         |
/// | `Option<Model>`     | Workbenches, object bodies, source files, if.       | `None`                  | Something is evaluated into a single model.                              |
/// |                     |                                                     |                         |
/// | `Models`            | Statement, statement list, body, multiplicities.    | `Models::default()`     | A collection of models . |
pub trait Eval<T = Value> {
    /// Evaluate a syntax element into a type `T`.
    fn eval(&self, context: &mut Context) -> EvalResult<T>;
}

impl MethodCall {
    /// Evaluate method call.
    ///
    /// Examples:
    /// ```microcad
    /// assert([2.0, 2.0].all_equal(), "All elements in this list must be equal.");
    /// ```
    fn eval(&self, context: &mut Context, lhs: &Expression) -> EvalResult<Value> {
        let value: Value = lhs.eval(context)?;
        value.call_method(&self.id, &self.argument_list, context)
    }
}

/// Like `todo!()` but within a evaluation context
///
/// emits a diagnostic error instead of panicking.
#[macro_export]
macro_rules! eval_todo {
    ($context: ident, $refer: ident, $($arg:tt)*) => {{
        $context.error($refer, EvalError::Todo(format_args!($($arg)*).to_string()))?;
        Ok(Value::None)
    }}
}

pub use eval_todo;
